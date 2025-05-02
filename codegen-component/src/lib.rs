use std::path::{Path, PathBuf};

pub use bumpalo;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KebabCodegenId(pub String);
impl From<String> for KebabCodegenId {
    fn from(s: String) -> Self {
        KebabCodegenId(s)
    }
}
impl From<&str> for KebabCodegenId {
    fn from(s: &str) -> Self {
        KebabCodegenId(s.to_string())
    }
}
impl std::fmt::Display for KebabCodegenId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Default)]
pub struct Component2SynPath(std::collections::HashMap<KebabCodegenId, syn::Path>);
// pub struct LanguagesMap(pub std::collections::HashMap<KebabCodegenId, Box<dyn std::any::Any>>);
// pub struct TargetsMap {
//     // pub languages: LanguagesMap,
//     pub targets: Vec<CodegenInstance>,
// }

pub struct CodegenInstance<'langs> {
    pub id: KebabCodegenId,
    pub generate: Box<dyn for<'a> Fn(&'a Component2SynPath) -> syn::ItemMod + 'langs>,
    pub external_deps: Vec<&'static str>,
    pub workspace_deps: Vec<(&'static str, &'static Path)>,
    pub codegen_deps: CgDepList<'langs>,
}

pub struct Crate<'langs> {
    pub id: KebabCodegenId,
    pub provides: Vec<CodegenInstance<'langs>>,
    pub global_workspace_deps: Vec<(&'static str, &'static Path)>,
}

#[must_use]
pub struct CgDepList<'langs> {
    codegen_deps: Vec<CodegenInstance<'langs>>,
}

impl<'langs> CgDepList<'langs> {
    pub fn new() -> Self {
        CgDepList {
            codegen_deps: Vec::new(),
        }
    }
    pub fn add(
        &mut self,
        dep: CodegenInstance<'langs>,
    ) -> Box<dyn for<'a> Fn(&'a Component2SynPath) -> syn::Path + 'static> {
        let id = dep.id.clone();
        self.codegen_deps.push(dep);
        Box::new(move |c2sp| {
            let path = c2sp
                .0
                .get(&id)
                .expect("dependency was not listed in the global dependency list");
            syn::parse_quote! {
                #path
            }
        })
    }
    pub fn self_path(&self) -> syn::Path {
        syn::parse_quote! {crate}
    }
    pub fn subtree(&self) -> CgDepList<'langs> {
        CgDepList {
            codegen_deps: vec![],
        }
    }
}

impl<'langs> Default for CgDepList<'langs> {
    fn default() -> Self {
        Self::new()
    }
}

const WORKSPACE_ROOT_RELPATH: &str = "../..";
const SIBLING_CRATES_RELPATH: &str = "..";

fn write_if_changed(path: &Path, content: &str) {
    if path.exists() {
        let existing_content = std::fs::read_to_string(path).unwrap();
        if existing_content == content {
            return;
        }
    }
    std::fs::write(path, content).unwrap();
}
impl<'langs> Crate<'langs> {
    pub fn generate(&self, base_path: &Path, c2sp: &mut Component2SynPath, other_crates: &[Crate]) {
        let crate_path = base_path.join(self.crate_relpath());
        std::fs::create_dir_all(&crate_path).unwrap();
        {
            let cargo_toml_path = crate_path.join("Cargo.toml");
            let cargo_toml_content = self.generate_cargo_toml(other_crates);
            write_if_changed(&cargo_toml_path, &cargo_toml_content);
        }
        {
            let src = crate_path.join("src");
            std::fs::create_dir_all(&src).unwrap();
            let contents = self.generate_dep_contents(c2sp);
            let content_names = contents
                .iter()
                .map(|content| content.ident.clone())
                .collect::<Vec<_>>();
            for content in contents {
                let file = syn::File {
                    shebang: None,
                    attrs: content
                        .attrs
                        .into_iter()
                        .map(|mut it| {
                            it.style = syn::AttrStyle::Inner(syn::token::Not(
                                proc_macro2::Span::call_site(),
                            ));
                            it
                        })
                        .collect(),
                    items: content.content.unwrap().1,
                };
                let file_path = src.join(format!("{}.rs", content.ident));
                write_if_changed(
                    &file_path,
                    &prettyplease::unparse(&syn_insert_use::insert_use(file)),
                );
            }
            let file_path = src.join("lib.rs");
            let lib: syn::File = syn::parse_quote! {
                #(
                    pub mod #content_names;
                )*
            };
            write_if_changed(
                &file_path,
                &prettyplease::unparse(&syn_insert_use::insert_use(lib)),
            );
        }
    }
    pub fn provides(&self, id: &KebabCodegenId) -> bool {
        self.provides.iter().any(|it| it.id == *id)
    }
    fn generate_dep_contents(&self, c2sp: &mut Component2SynPath) -> Vec<syn::ItemMod> {
        let mut current_c2sp = Component2SynPath(c2sp.0.clone());
        let mut contents = vec![];
        for dep in self.provides.iter() {
            self.generate_single_dep_contents(dep, &mut current_c2sp, c2sp, &mut contents);
        }
        contents
    }
    fn generate_single_dep_contents(
        &self,
        dep: &CodegenInstance<'_>,
        current_c2sp: &mut Component2SynPath,
        global_c2sp: &mut Component2SynPath,
        contents: &mut Vec<syn::ItemMod>,
    ) {
        dbg!(&dep.id);
        if current_c2sp.0.contains_key(&dep.id) {
            return;
        }
        for depdep in dep.codegen_deps.codegen_deps.iter() {
            println!("dep: {} -> {}", dep.id, depdep.id);
            self.generate_single_dep_contents(depdep, current_c2sp, global_c2sp, contents);
        }
        let m = (dep.generate)(current_c2sp);
        let dep_ident = &m.ident;
        current_c2sp.0.insert(
            dep.id.clone(),
            syn::parse_quote! {
                crate::#dep_ident
            },
        );
        let crate_ident = self.crate_ident();
        global_c2sp.0.insert(
            dep.id.clone(),
            syn::parse_quote! {
                #crate_ident::#dep_ident
            },
        );
        dbg!(&m.ident);
        contents.push(m);
    }
    fn crate_ident(&self) -> syn::Ident {
        syn::Ident::new(&self.id.0.replace("-", "_"), proc_macro2::Span::call_site())
    }
    fn crate_relpath(&self) -> PathBuf {
        self.id.0.as_str().into()
    }
    fn generate_cargo_toml(&self, other_crates: &[Crate]) -> String {
        let depends_on_crates = other_crates
            .iter()
            .filter(|it| {
                self.provides
                    .iter()
                    .flat_map(|it| it.codegen_deps.codegen_deps.iter())
                    .any(|dep| it.provides(&dep.id))
                    && it.id != self.id
            })
            .collect::<Vec<_>>();
        let workspace_dep2entry = |dep: &(&str, &Path)| {
            (
                dep.0.to_string(),
                toml::Value::Table(
                    [(
                        "path".into(),
                        toml::Value::String(
                            PathBuf::from(WORKSPACE_ROOT_RELPATH)
                                .join(dep.1)
                                .join(dep.0)
                                .to_string_lossy()
                                .into(),
                        ),
                    )]
                    .into_iter()
                    .collect::<toml::Table>(),
                ),
            )
        };
        let cargo_toml = [
            (
                String::from("package"),
                toml::Value::Table(
                    [
                        ("name".into(), toml::Value::String(self.id.0.clone())),
                        ("version".into(), toml::Value::String("0.0.0".into())),
                        ("edition".into(), toml::Value::String("2024".into())),
                    ]
                    .into_iter()
                    .collect::<toml::Table>(),
                ),
            ),
            (
                String::from("dependencies"),
                toml::Value::Table(
                    self.global_workspace_deps
                        .iter()
                        .map(workspace_dep2entry)
                        .chain(self.provides.iter().flat_map(|it| {
                            it.external_deps
                                .iter()
                                .map(|dep| {
                                    (
                                        dep.to_string(),
                                        toml::Value::Table(
                                            [("workspace".into(), toml::Value::Boolean(true))]
                                                .into_iter()
                                                .collect::<toml::Table>(),
                                        ),
                                    )
                                })
                                .chain(it.workspace_deps.iter().map(workspace_dep2entry))
                        }))
                        .chain(depends_on_crates.iter().map(|dep| {
                            (
                                dep.id.to_string(),
                                toml::Value::Table(
                                    [(
                                        "path".into(),
                                        toml::Value::String(
                                            PathBuf::from(SIBLING_CRATES_RELPATH)
                                                .join(dep.crate_relpath())
                                                .to_string_lossy()
                                                .into(),
                                        ),
                                    )]
                                    .into_iter()
                                    .collect::<toml::Table>(),
                                ),
                            )
                        }))
                        .collect::<toml::Table>(),
                ),
            ),
        ]
        .into_iter()
        .collect::<toml::Table>();
        toml::to_string(&cargo_toml).unwrap()
    }
}
