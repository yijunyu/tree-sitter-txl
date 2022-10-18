(source_file [0, 0] - [33, 0]
  (line_comment [0, 0] - [0, 66])
  (line_comment [1, 0] - [1, 63])
  (line_comment [2, 0] - [2, 46])
  (function_item [3, 0] - [5, 1]
    name: (identifier [3, 3] - [3, 13])
    type_parameters: (type_parameters [3, 13] - [3, 21]
      (lifetime [3, 14] - [3, 16]
        (identifier [3, 15] - [3, 16]))
      (lifetime [3, 18] - [3, 20]
        (identifier [3, 19] - [3, 20])))
    parameters: (parameters [3, 21] - [3, 45]
      (parameter [3, 22] - [3, 32]
        pattern: (identifier [3, 22] - [3, 23])
        type: (reference_type [3, 25] - [3, 32]
          (lifetime [3, 26] - [3, 28]
            (identifier [3, 27] - [3, 28]))
          type: (primitive_type [3, 29] - [3, 32])))
      (parameter [3, 34] - [3, 44]
        pattern: (identifier [3, 34] - [3, 35])
        type: (reference_type [3, 37] - [3, 44]
          (lifetime [3, 38] - [3, 40]
            (identifier [3, 39] - [3, 40]))
          type: (primitive_type [3, 41] - [3, 44]))))
    body: (block [3, 46] - [5, 1]
      (macro_invocation [4, 4] - [4, 41]
        macro: (identifier [4, 4] - [4, 11])
        (token_tree [4, 12] - [4, 41]
          (string_literal [4, 13] - [4, 34])
          (identifier [4, 36] - [4, 37])
          (identifier [4, 39] - [4, 40])))))
  (line_comment [7, 0] - [7, 74])
  (function_item [8, 0] - [16, 1]
    name: (identifier [8, 3] - [8, 16])
    type_parameters: (type_parameters [8, 16] - [8, 20]
      (lifetime [8, 17] - [8, 19]
        (identifier [8, 18] - [8, 19])))
    parameters: (parameters [8, 20] - [8, 22])
    body: (block [8, 23] - [16, 1]
      (let_declaration [9, 4] - [9, 16]
        pattern: (identifier [9, 8] - [9, 10])
        value: (integer_literal [9, 13] - [9, 15]))
      (line_comment [11, 4] - [11, 44])
      (let_declaration [12, 4] - [12, 25]
        pattern: (identifier [12, 8] - [12, 9])
        type: (reference_type [12, 11] - [12, 18]
          (lifetime [12, 12] - [12, 14]
            (identifier [12, 13] - [12, 14]))
          type: (primitive_type [12, 15] - [12, 18]))
        value: (reference_expression [12, 21] - [12, 24]
          value: (identifier [12, 22] - [12, 24])))
      (line_comment [13, 4] - [13, 74])
      (line_comment [14, 4] - [14, 77])
      (line_comment [15, 4] - [15, 78])))
  (function_item [18, 0] - [32, 1]
    name: (identifier [18, 3] - [18, 7])
    parameters: (parameters [18, 7] - [18, 9])
    body: (block [18, 10] - [32, 1]
      (line_comment [19, 4] - [19, 45])
      (let_declaration [20, 4] - [20, 30]
        pattern: (tuple_pattern [20, 8] - [20, 20]
          (identifier [20, 9] - [20, 13])
          (identifier [20, 15] - [20, 19]))
        value: (tuple_expression [20, 23] - [20, 29]
          (integer_literal [20, 24] - [20, 25])
          (integer_literal [20, 27] - [20, 28])))
      (line_comment [22, 4] - [22, 68])
      (call_expression [23, 4] - [23, 28]
        function: (identifier [23, 4] - [23, 14])
        arguments: (arguments [23, 14] - [23, 28]
          (reference_expression [23, 15] - [23, 20]
            value: (identifier [23, 16] - [23, 20]))
          (reference_expression [23, 22] - [23, 27]
            value: (identifier [23, 23] - [23, 27]))))
      (line_comment [24, 4] - [24, 62])
      (line_comment [25, 4] - [25, 62])
      (line_comment [26, 4] - [26, 43])
      (call_expression [28, 4] - [28, 19]
        function: (identifier [28, 4] - [28, 17])
        arguments: (arguments [28, 17] - [28, 19]))
      (line_comment [29, 4] - [29, 66])
      (line_comment [30, 4] - [30, 68])
      (line_comment [31, 4] - [31, 75]))))
