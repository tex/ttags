;; This is from tree-sitter-cpp/queries/tags.scm

(struct_specifier
  name: (type_identifier) @name
  body: (_)
) @definition.d1

(declaration
  type: (union_specifier
          name: (type_identifier) @name)) @definition.d2

(function_declarator
  declarator: (identifier) @name) @definition.d3

(function_declarator
  declarator: (field_identifier) @name) @definition.d4

(function_declarator
  declarator: (qualified_identifier
                name: (identifier) @name)) @definition.d5

(type_definition 
  declarator: (type_identifier) @name) @definition.d6

(enum_specifier
  name: (type_identifier) @name) @definition.d7

(class_specifier
  name: (type_identifier) @name
  body: (_)
) @definition.d8

;; ---

;; Definitions

;; typedef void (*SomeHandsomeFnc)(const char* file, int32_t line, intptr_t arg1, intptr_t arg2);
(type_definition
  (function_declarator
    (parenthesized_declarator
      (pointer_declarator
        declarator: (type_identifier) @name)))) @definition.d9

(preproc_def
  name: (identifier) @name) @definition.d10
(preproc_function_def
  name: (identifier) @name) @definition.d11

;;(namespace_definition
;;  name: (identifier) @name) @definition.namespace

(field_declaration
  declarator: (field_identifier) @name) @definition.d12

(enumerator
  name: (identifier) @name) @definition.d13

;;(declaration
;;  (init_declarator
;;    (pointer_declarator
;;      declarator: (identifier) @name))) @definition.identifier
;;

;; Constants... oh, no way to know if it is local or global...
;; probably better to have it than not have it.

;; const uint8_t MY_CONSTANT = 20;
(declaration
  (init_declarator
      declarator: (identifier) @name)) @definition.d14
(declaration
  (init_declarator
      (qualified_identifier
        name: (identifier) @name))) @definition.d15

;; const char MY_ARRAY[] = "Hello";
(declaration
  (init_declarator
    (array_declarator
      declarator: (identifier) @name))) @definition.d16
;; extern uint16_t Test
(declaration
  declarator: (identifier) @name) @definition.d17
;; extern SomeClass Test
(declaration
    (qualified_identifier
      name: (identifier) @name)) @definition.d18
;; extern SomeClass Test[Z]
(declaration
  (array_declarator
    (qualified_identifier
      name: (identifier) @name))) @definition.d19

;; References

(qualified_identifier
  name: (type_identifier) @name) @reference.r1

;;(call_expression
;;  function: (qualified_identifier) @name) @reference.call

(call_expression
  function: (identifier) @name) @reference.r2

;; xyz(TEST)
(call_expression
  (argument_list
    (identifier) @name)) @reference.r3

;; xyz(&TEST)
(call_expression
  (argument_list
    (pointer_expression
      (identifier) @name))) @reference.r4

;; &TEST[i]
(pointer_expression
  (subscript_expression
    (identifier) @name)) @reference.r5

;; XXX::Test()
(call_expression
  (qualified_identifier
    name: (identifier) @name)) @reference.r6

;; XXX::YYY() : ZZZ(BBB:TEST) {}
;; How to do proper recursion?
(field_initializer
  (argument_list
    (qualified_identifier
      name: (identifier) @name)) @reference.r7)
(field_initializer
  (argument_list
    (qualified_identifier
      (qualified_identifier
        name: (identifier) @name))) @reference.r8)

;; How to do proper recursion?
(field_initializer
  (argument_list
    (qualified_identifier
      scope: (namespace_identifier) @name)) @reference.r9)
(field_initializer
  (argument_list
    (qualified_identifier
      (qualified_identifier
        scope: (namespace_identifier) @name))) @reference.r10)

;; m_Struct_1.XXX();
;;(call_expression
;;  (field_expression
;;    argument: (identifier) @name)) @reference.call

;; XXX.Test();
(call_expression
  (field_expression
    field: (field_identifier) @name)) @reference.r11

;; MyStruct XXX;
(declaration
  type: (type_identifier) @name) @reference.r12

(declaration
  type: (qualified_identifier) @name) @reference.r13

(declaration
  (init_declarator
    (argument_list
      (qualified_identifier) @name))) @reference.r14

(field_declaration
  type: (type_identifier) @name) @reference.r15

(field_declaration
  type: (qualified_identifier) @name) @reference.r16

(base_class_clause
  (qualified_identifier) @name) @reference.r17

(base_class_clause
  (type_identifier) @name) @reference.r18

(parameter_declaration
  type: (_) @name
  declarator: (identifier)) @reference.r19

;; This captures everything in conditional clausule which is not really optimal
;;(condition_clause
;;  value: (_) @name) @reference.r20

(cast_expression
  (type_descriptor
    type: (_) @name)) @reference.r21

;; X.test = Y
(assignment_expression
  (field_expression
    field: (field_identifier) @name)) @reference.r22

(case_statement [
  value: (identifier) @name
  value: (qualified_identifier [
           (identifier) @name
           (qualified_identifier (identifier) @name)
           (qualified_identifier (qualified_identifier (identifier) @name))
           (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))))
           ]) @doc
]) @reference.case_statement.identifier

(case_statement [
  value: (identifier) @name
  value: (qualified_identifier [
           (namespace_identifier) @name
           (qualified_identifier (namespace_identifier) @name)
           (qualified_identifier (qualified_identifier (namespace_identifier) @name))
           (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name)))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name)))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name))))))
           ]) @doc
]) @reference.case_statement.namespace_identifier


(binary_expression
  left: (identifier) @name) @reference.r25
(binary_expression
  right: (identifier) @name) @reference.r26

;; XYZ_TYPE FunctionName(something) { ... }
(function_definition
  type: (type_identifier) @name) @reference.r27

(new_expression
  type: (type_identifier) @name) @reference.r28

