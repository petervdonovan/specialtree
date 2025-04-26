use langspec::humanreadable::LangSpecHuman;

pub fn fib() -> LangSpecHuman<tymetafuncspec_core::Core> {
    serde_json::from_str(
        r#"
    {
        "name": {
            "human": "fib",
            "camel": "Fib",
            "snake": "fib"
        },
        "products": [
            {
                "name": {
                    "human": "f",
                    "camel": "F",
                    "snake": "f"
                },
                "sorts": [
                    {"Algebraic": {"Sum": "ℕ"}}
                ]
            },
            {
                "name": {
                    "human": "+",
                    "camel": "Plus",
                    "snake": "plus"
                },
                "sorts": [
                    {"Algebraic": {"Product": "LeftOperand"}},
                    {"Algebraic": {"Product": "RightOperand"}}
                ]
            },
            {
                "name": {
                    "human": "LeftOperand",
                    "camel": "LeftOperand",
                    "snake": "left_operand"
                },
                "sorts": [
                    {"Algebraic": {"Sum": "ℕ"}}
                ]
            },
            {
                "name": {
                    "human": "RightOperand",
                    "camel": "RightOperand",
                    "snake": "right_operand"
                },
                "sorts": [
                    {"Algebraic": {"Sum": "ℕ"}}
                ]
            },
            {
                "name": {
                    "human": "∑",
                    "camel": "Sum",
                    "snake": "sum"
                },
                "sorts": [
                    {"TyMetaFunc": {"f": 1, "a": [{"Algebraic": {"Sum": "ℕ"}}]}}
                ]
            }
        ],
        "sums": [
            {
                "name": {
                    "human": "ℕ",
                    "camel": "Nat",
                    "snake": "nat"
                },
                "sorts": [
                    {"TyMetaFunc": {"f": 0, "a": []}},
                    {"Algebraic": {"Product": "f"}},
                    {"Algebraic": {"Product": "+"}},
                    {"Algebraic": {"Product": "∑"}}
                ]
            }
        ]
    }
    "#,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        let lsh = fib();
        let expected = expect_test::expect![[r#"
            {
              "name": {
                "human": "fib",
                "camel": "Fib",
                "snake": "fib"
              },
              "products": [
                {
                  "name": {
                    "human": "f",
                    "camel": "F",
                    "snake": "f"
                  },
                  "sorts": [
                    {
                      "Algebraic": {
                        "Sum": "ℕ"
                      }
                    }
                  ]
                },
                {
                  "name": {
                    "human": "+",
                    "camel": "Plus",
                    "snake": "plus"
                  },
                  "sorts": [
                    {
                      "Algebraic": {
                        "Product": "LeftOperand"
                      }
                    },
                    {
                      "Algebraic": {
                        "Product": "RightOperand"
                      }
                    }
                  ]
                },
                {
                  "name": {
                    "human": "LeftOperand",
                    "camel": "LeftOperand",
                    "snake": "left_operand"
                  },
                  "sorts": [
                    {
                      "Algebraic": {
                        "Sum": "ℕ"
                      }
                    }
                  ]
                },
                {
                  "name": {
                    "human": "RightOperand",
                    "camel": "RightOperand",
                    "snake": "right_operand"
                  },
                  "sorts": [
                    {
                      "Algebraic": {
                        "Sum": "ℕ"
                      }
                    }
                  ]
                },
                {
                  "name": {
                    "human": "∑",
                    "camel": "Sum",
                    "snake": "sum"
                  },
                  "sorts": [
                    {
                      "TyMetaFunc": {
                        "f": 1,
                        "a": [
                          {
                            "Algebraic": {
                              "Sum": "ℕ"
                            }
                          }
                        ]
                      }
                    }
                  ]
                }
              ],
              "sums": [
                {
                  "name": {
                    "human": "ℕ",
                    "camel": "Nat",
                    "snake": "nat"
                  },
                  "sorts": [
                    {
                      "TyMetaFunc": {
                        "f": 0,
                        "a": []
                      }
                    },
                    {
                      "TyMetaFunc": {
                        "f": 3,
                        "a": [
                          {
                            "Algebraic": {
                              "Product": "f"
                            }
                          }
                        ]
                      }
                    },
                    {
                      "TyMetaFunc": {
                        "f": 3,
                        "a": [
                          {
                            "Algebraic": {
                              "Product": "+"
                            }
                          }
                        ]
                      }
                    },
                    {
                      "TyMetaFunc": {
                        "f": 3,
                        "a": [
                          {
                            "Algebraic": {
                              "Product": "∑"
                            }
                          }
                        ]
                      }
                    }
                  ]
                }
              ]
            }"#]];
        expected.assert_eq(&serde_json::to_string_pretty(&lsh).unwrap());
    }
}
