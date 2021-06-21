use crate::vm::VM;
use std::fs;

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  fn test(location: &str) {
    let contents = fs::read_to_string(location).expect("Something went wrong reading the file");
    let v: serde_json::Value = serde_json::from_str(&contents).unwrap();
    let instructions = v.as_object().unwrap()["input_program"]
      .as_array()
      .unwrap()
      .to_vec();

    let mut vm = crate::vm::VM::new();
    vm.load(instructions);

    if v.as_object().unwrap().contains_key("initial_context") {
      let ctx = v.as_object().unwrap()["initial_context"]
        .as_object()
        .unwrap();
      for k in ctx.keys() {
        let v = ctx.get(k).unwrap();
        if v.is_string() {
          vm.stack
            .push(crate::vm::Value::String(v.as_str().unwrap().to_string()));
          vm.stack
            .push(crate::vm::Value::String(k.as_str().to_string()));
          vm.i_setcontext();
        } else if v.is_number() {
          vm.stack.push(crate::vm::Value::Number(v.as_f64().unwrap()));
          vm.stack
            .push(crate::vm::Value::String(k.as_str().to_string()));
          vm.i_setcontext();
        }
      }
    }

    vm.run();

    println!("\nStack at end: {:?}", vm.stack);

    let expected = v.as_object().unwrap()["expected"].as_object().unwrap();
    if expected.contains_key("stack") {
      let stack = expected["stack"].as_array().unwrap();
      let mut i = 0;
      for s in stack {
        match &vm.stack[i as usize] {
          crate::vm::Value::Number(val) => {
            assert_eq!(*val, s.as_f64().unwrap());
          }
          crate::vm::Value::String(val) => {
            assert_eq!(val, s.as_str().unwrap());
          }
        }
        i += 1;
      }
    }
  }

  #[test]
  fn test_and_0() {
    test("./src/tests/and_0.json");
  }
  #[test]
  fn test_and_1() {
    test("./src/tests/and_1.json");
  }
  #[test]
  fn test_and_2() {
    test("./src/tests/and_2.json");
  }
  #[test]
  fn test_and_3() {
    test("./src/tests/and_3.json");
  }
  #[test]
  fn test_and_4() {
    test("./src/tests/and_4.json");
  }
  #[test]
  fn test_and_5() {
    test("./src/tests/and_5.json");
  }
  #[test]
  fn test_braces() {
    test("./src/tests/braces.json");
  }
  #[test]
  fn test_braces_nested() {
    test("./src/tests/braces_nested.json");
  }
  #[test]
  fn test_concat_0() {
    test("./src/tests/concat_0.json");
  }
  #[test]
  fn test_concat_1() {
    test("./src/tests/concat_1.json");
  }
  #[test]
  fn test_concat_2() {
    test("./src/tests/concat_2.json");
  }
  #[test]
  fn test_concat_3() {
    test("./src/tests/concat_3.json");
  }
  #[test]
  fn test_delContext() {
    test("./src/tests/delContext.json");
  }
  #[test]
  fn test_dup_0() {
    test("./src/tests/dup_0.json");
  }
  #[test]
  fn test_dup_1() {
    test("./src/tests/dup_1.json");
  }
  #[test]
  fn test_dup_2() {
    test("./src/tests/dup_2.json");
  }
  #[test]
  fn test_dup_3() {
    test("./src/tests/dup_3.json");
  }
  #[test]
  fn test_eq_0() {
    test("./src/tests/eq_0.json");
  }
  #[test]
  fn test_eq_1() {
    test("./src/tests/eq_1.json");
  }
  #[test]
  fn test_eq_2() {
    test("./src/tests/eq_2.json");
  }
  #[test]
  fn test_eq_3() {
    test("./src/tests/eq_3.json");
  }
  #[test]
  fn test_eq_4() {
    test("./src/tests/eq_4.json");
  }
  #[test]
  fn test_eq_5() {
    test("./src/tests/eq_5.json");
  }
  #[test]
  fn test_eq_6() {
    test("./src/tests/eq_6.json");
  }
  #[test]
  fn test_example_0() {
    test("./src/tests/example_0.json");
  }
  #[test]
  fn test_example_1() {
    test("./src/tests/example_1.json");
  }
  #[test]
  fn test_example_2() {
    test("./src/tests/example_2.json");
  }
  #[test]
  fn test_example_3() {
    test("./src/tests/example_3.json");
  }
  #[test]
  fn test_example_4() {
    test("./src/tests/example_4.json");
  }
  #[test]
  fn test_getContext_1() {
    test("./src/tests/getContext_1.json");
  }
  #[test]
  fn test_getContext_2() {
    test("./src/tests/getContext_2.json");
  }
  #[test]
  fn test_getContext_3() {
    test("./src/tests/getContext_3.json");
  }
  #[test]
  fn test_goto_by_label() {
    test("./src/tests/goto_by_label.json");
  }
  #[test]
  fn test_goto_by_number() {
    test("./src/tests/goto_by_number.json");
  }
  #[test]
  fn test_gt_0() {
    test("./src/tests/gt_0.json");
  }
  #[test]
  fn test_gt_1() {
    test("./src/tests/gt_1.json");
  }
  #[test]
  fn test_gt_2() {
    test("./src/tests/gt_2.json");
  }
  #[test]
  fn test_gt_3() {
    test("./src/tests/gt_3.json");
  }
  #[test]
  fn test_gt_4() {
    test("./src/tests/gt_4.json");
  }
  #[test]
  fn test_gt_5() {
    test("./src/tests/gt_5.json");
  }
  #[test]
  fn test_gt_6() {
    test("./src/tests/gt_6.json");
  }
  #[test]
  fn test_gt_7() {
    test("./src/tests/gt_7.json");
  }
  #[test]
  fn test_hasContext_1() {
    test("./src/tests/hasContext_1.json");
  }
  #[test]
  fn test_hasContext_2() {
    test("./src/tests/hasContext_2.json");
  }
  #[test]
  fn test_hasContext_3() {
    test("./src/tests/hasContext_3.json");
  }
  #[test]
  fn test_if_jgz_0() {
    test("./src/tests/if_jgz_0.json");
  }
  #[test]
  fn test_if_jgz_1() {
    test("./src/tests/if_jgz_1.json");
  }
  #[test]
  fn test_jgz_0() {
    test("./src/tests/jgz_0.json");
  }
  #[test]
  fn test_jgz_1() {
    test("./src/tests/jgz_1.json");
  }
  #[test]
  fn test_jgz_2() {
    test("./src/tests/jgz_2.json");
  }
  #[test]
  fn test_jz_0() {
    test("./src/tests/jz_0.json");
  }
  #[test]
  fn test_jz_1() {
    test("./src/tests/jz_1.json");
  }
  #[test]
  fn test_jz_2() {
    test("./src/tests/jz_2.json");
  }
  #[test]
  fn test_lt_0() {
    test("./src/tests/lt_0.json");
  }
  #[test]
  fn test_lt_1() {
    test("./src/tests/lt_1.json");
  }
  #[test]
  fn test_lt_2() {
    test("./src/tests/lt_2.json");
  }
  #[test]
  fn test_lt_3() {
    test("./src/tests/lt_3.json");
  }
  #[test]
  fn test_lt_4() {
    test("./src/tests/lt_4.json");
  }
  #[test]
  fn test_lt_5() {
    test("./src/tests/lt_5.json");
  }
  #[test]
  fn test_lt_6() {
    test("./src/tests/lt_6.json");
  }
  #[test]
  fn test_lt_7() {
    test("./src/tests/lt_7.json");
  }
  #[test]
  fn test_min_0() {
    test("./src/tests/min_0.json");
  }
  #[test]
  fn test_min_1() {
    test("./src/tests/min_1.json");
  }
  #[test]
  fn test_min_2() {
    test("./src/tests/min_2.json");
  }
  #[test]
  fn test_multiply_0() {
    test("./src/tests/multiply_0.json");
  }
  #[test]
  fn test_multiply_1() {
    test("./src/tests/multiply_1.json");
  }
  #[test]
  fn test_nop_0() {
    test("./src/tests/nop_0.json");
  }
  #[test]
  fn test_nop_1() {
    test("./src/tests/nop_1.json");
  }
  #[test]
  fn test_not_0() {
    test("./src/tests/not_0.json");
  }
  #[test]
  fn test_not_1() {
    test("./src/tests/not_1.json");
  }
  #[test]
  fn test_not_2() {
    test("./src/tests/not_2.json");
  }
  #[test]
  fn test_or_0() {
    test("./src/tests/or_0.json");
  }
  #[test]
  fn test_or_1() {
    test("./src/tests/or_1.json");
  }
  #[test]
  fn test_or_2() {
    test("./src/tests/or_2.json");
  }
  #[test]
  fn test_or_3() {
    test("./src/tests/or_3.json");
  }
  #[test]
  fn test_or_4() {
    test("./src/tests/or_4.json");
  }
  #[test]
  fn test_or_5() {
    test("./src/tests/or_5.json");
  }
  #[test]
  fn test_plus_0() {
    test("./src/tests/plus_0.json");
  }
  #[test]
  fn test_plus_1() {
    test("./src/tests/plus_1.json");
  }
  #[test]
  fn test_poor_mans_function() {
    test("./src/tests/poor_mans_function.json");
  }
  #[test]
  fn test_pop_0() {
    test("./src/tests/pop_0.json");
  }
  #[test]
  fn test_pop_1() {
    test("./src/tests/pop_1.json");
  }
  #[test]
  fn test_pop_2() {
    test("./src/tests/pop_2.json");
  }
  #[test]
  fn test_pop_3() {
    test("./src/tests/pop_3.json");
  }
  #[test]
  fn test_ppc_0() {
    test("./src/tests/ppc_0.json");
  }
  #[test]
  fn test_ppc_1() {
    test("./src/tests/ppc_1.json");
  }
  #[test]
  fn test_push_negative_numbers() {
    test("./src/tests/push_negative_numbers.json");
  }
  #[test]
  fn test_push_numbers() {
    test("./src/tests/push_numbers.json");
  }
  #[test]
  fn test_push_strings() {
    test("./src/tests/push_strings.json");
  }
  #[test]
  fn test_rconcat_0() {
    test("./src/tests/rconcat_0.json");
  }
  #[test]
  fn test_rconcat_1() {
    test("./src/tests/rconcat_1.json");
  }
  #[test]
  fn test_rconcat_2() {
    test("./src/tests/rconcat_2.json");
  }
  #[test]
  fn test_rconcat_3() {
    test("./src/tests/rconcat_3.json");
  }
  #[test]
  fn test_setContext_number() {
    test("./src/tests/setContext_number.json");
  }
  #[test]
  fn test_setContext_string() {
    test("./src/tests/setContext_string.json");
  }
  #[test]
  fn test_stacksize_0() {
    test("./src/tests/stacksize_0.json");
  }
  #[test]
  fn test_stacksize_1() {
    test("./src/tests/stacksize_1.json");
  }
  #[test]
  fn test_stdout_0() {
    test("./src/tests/stdout_0.json");
  }
  #[test]
  fn test_stdout_1() {
    test("./src/tests/stdout_1.json");
  }
}
