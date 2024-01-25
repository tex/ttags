;; This is from tree-sitter-cpp/queries/tags.scm

(struct_specifier
  name: (type_identifier) @name body:(_)) @definition.class

(declaration
  type: (union_specifier
          name: (type_identifier) @name)) @definition.class

(function_declarator
  declarator: (identifier) @name) @definition.function

(function_declarator
  declarator: (field_identifier) @name) @definition.function

(function_declarator
  declarator: (qualified_identifier
                scope: (namespace_identifier) @local.scope
                name: (identifier) @name)) @definition.method

(type_definition 
  declarator: (type_identifier) @name) @definition.type

(enum_specifier
  name: (type_identifier) @name) @definition.type

(class_specifier
  name: (type_identifier) @name) @definition.class

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

;; References

;;(call_expression
;;  function: (qualified_identifier) @name) @reference.call

(call_expression
  function: (identifier) @name) @reference.call

;; xyz(TEST)
(call_expression
  (argument_list
    (identifier) @name)) @reference.identifier

;; XXX::Test()
(call_expression
  (qualified_identifier
    name: (identifier) @name)) @reference.call

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
