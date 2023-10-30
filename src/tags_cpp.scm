
;; Definitions

(preproc_def
  name: (identifier) @name) @definition.macro

(function_definition
  (function_declarator
    declarator: (_) @name)) @definition.method

(function_declarator
  declarator: (field_identifier) @name) @definition.method

(namespace_definition
  name: (identifier) @name) @definition.namespace

(struct_specifier
  name: (type_identifier) @name) @definition.class

(field_declaration
  declarator: (field_identifier) @name) @definition.field

(enum_specifier
  name: (type_identifier) @name) @definition.enum

(enumerator
  name: (identifier) @name) @definition.identifier

(class_specifier
  name: (type_identifier) @name) @definition.class

(declaration
  (init_declarator
    (pointer_declarator
      declarator: (identifier) @name))) @definition.identifier

(declaration
  (init_declarator
      declarator: (identifier) @name)) @definition.identifier

;; References

(call_expression
  function: (qualified_identifier) @name) @reference.call

(call_expression
  function: (identifier) @name) @reference.call

(call_expression
  (field_expression
    field: (field_identifier) @name)) @reference.call

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

