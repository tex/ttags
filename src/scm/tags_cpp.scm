;; This is from tree-sitter-cpp/queries/tags.scm

(struct_specifier
  name: (type_identifier) @name
  body: (_)
) @definition.class

(declaration
  type: (union_specifier
          name: (type_identifier) @name)) @definition.class

(function_declarator
  declarator: (identifier) @name) @definition.function

(function_declarator
  declarator: (field_identifier) @name) @definition.function

(function_declarator
  declarator: (qualified_identifier
                name: (identifier) @name)) @definition.function

(type_definition 
  declarator: (type_identifier) @name) @definition.type

(enum_specifier
  name: (type_identifier) @name) @definition.type

(class_specifier
  name: (type_identifier) @name
  body: (_)
) @definition.class

;; ---

;; Definitions

;; typedef void (*SomeHandsomeFnc)(const char* file, int32_t line, intptr_t arg1, intptr_t arg2);
(type_definition
  (function_declarator
    (parenthesized_declarator
      (pointer_declarator
        declarator: (type_identifier) @name)))) @definition.type

(preproc_def
  name: (identifier) @name) @definition.macro
(preproc_function_def
  name: (identifier) @name) @definition.macro

;;(namespace_definition
;;  name: (identifier) @name) @definition.namespace

(field_declaration
  declarator: (field_identifier) @name) @definition.field

(enumerator
  name: (identifier) @name) @definition.identifier

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
      declarator: (identifier) @name)) @definition.identifier
;; const char MY_ARRAY[] = "Hello";
(declaration
  (init_declarator
    (array_declarator
      declarator: (identifier) @name))) @definition.identifier
;; extern uint16_t Test
(declaration
  declarator: (identifier) @name) @definition.identifier
;; extern SomeClass Test
(declaration
    (qualified_identifier
      name: (identifier) @name)) @definition.identifier
;; extern SomeClass Test[Z]
(declaration
  (array_declarator
    (qualified_identifier
      name: (identifier) @name))) @definition.identifier

;; References

(qualified_identifier
  name: (type_identifier) @name) @reference.identifier

;;(call_expression
;;  function: (qualified_identifier) @name) @reference.call

(call_expression
  function: (identifier) @name) @reference.call

;; xyz(TEST)
(call_expression
  (argument_list
    (identifier) @name)) @reference.identifier

;; xyz(&TEST)
(call_expression
  (argument_list
    (pointer_expression
      (identifier) @name))) @reference.identifier

;; &TEST[i]
(pointer_expression
  (subscript_expression
    (identifier) @name)) @reference.identifier

;; XXX::Test()
(call_expression
  (qualified_identifier
    name: (identifier) @name)) @reference.call

;; XXX::YYY() : ZZZ(BBB:TEST) {}
;; How to do proper recursion?
(field_initializer
  (argument_list
    (qualified_identifier
      name: (identifier) @name)) @reference.identifier)
(field_initializer
  (argument_list
    (qualified_identifier
      (qualified_identifier
        name: (identifier) @name))) @reference.identifier)

;; How to do proper recursion?
(field_initializer
  (argument_list
    (qualified_identifier
      scope: (namespace_identifier) @name)) @reference.identifier)
(field_initializer
  (argument_list
    (qualified_identifier
      (qualified_identifier
        scope: (namespace_identifier) @name))) @reference.identifier)

;; m_Struct_1.XXX();
;;(call_expression
;;  (field_expression
;;    argument: (identifier) @name)) @reference.call

;; XXX.Test();
(call_expression
  (field_expression
    field: (field_identifier) @name)) @reference.call

;; MyStruct XXX;
(declaration
  type: (type_identifier) @name) @reference.class

(declaration
  type: (qualified_identifier) @name) @reference.class

(declaration
  (init_declarator
    (argument_list
      (qualified_identifier) @name))) @reference.class

(field_declaration
  type: (type_identifier) @name) @reference.class

(field_declaration
  type: (qualified_identifier) @name) @reference.class

(base_class_clause
  (qualified_identifier) @name) @reference.class

(base_class_clause
  (type_identifier) @name) @reference.class

(parameter_declaration
  type: (_) @name
  declarator: (identifier)) @reference.class

(condition_clause
  value: (_) @name) @reference.identifier

(cast_expression
  (type_descriptor
    type: (_) @name)) @reference.class

;; X.test = Y
(assignment_expression
  (field_expression
    field: (field_identifier) @name)) @reference.identifier

;; case something:
(case_statement
  value: (identifier) @name) @reference.identifier
(case_statement
  value: (qualified_identifier) @name) @reference.qualified_identifier


(binary_expression
  left: (identifier) @name) @reference.identifier
(binary_expression
  right: (identifier) @name) @reference.identifier

;; XYZ_TYPE FunctionName(something) { ... }
(function_definition
  type: (type_identifier) @name) @reference.identifier

(new_expression
  type: (type_identifier) @name) @reference.identifier

