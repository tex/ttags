;; Definitions

;; This is from tree-sitter-cpp/queries/tags.scm

(struct_specifier
  name: (type_identifier) @name
  body: (_)
) @definition.d1

(declaration
  type: (union_specifier
          name: (type_identifier) @name)) @definition.declaration.union_specifier

(function_declarator
  declarator: (field_identifier) @name) @definition.function_declarator.field_identifier

(function_declarator [
  declarator: (identifier) @name
  declarator: (qualified_identifier [
                (identifier) @name
                (qualified_identifier (identifier) @name)
                (qualified_identifier (qualified_identifier (identifier) @name))
                (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))
                (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))
                (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))))
                (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))))
         ]) @doc
]) @definition.function_declarator.identifier

(template_declaration
  (function_definition
    declarator: (function_declarator
      declarator: [
        (template_function (identifier) @name)
        (qualified_identifier (template_function (identifier) @name))
        (qualified_identifier (qualified_identifier (template_function (identifier) @name)))
        (qualified_identifier (qualified_identifier (qualified_identifier (template_function (identifier) @name))))
        (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (template_function (identifier) @name)))))
        (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (template_function (identifier) @name))))))
        (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (template_function (identifier) @name)))))))
  ]) @doc
)) @definition.template_function_definition.identifier

(type_definition 
  declarator: (type_identifier) @name) @definition.d6

(enum_specifier
  name: (type_identifier) @name) @definition.d7

(class_specifier
  name: (type_identifier) @name
  body: (_)
) @definition.d8

;; ---

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

(field_declaration [
  (field_identifier) @name
  (array_declarator declarator: (field_identifier) @name)
  (pointer_declarator declarator: (field_identifier) @name)
  (pointer_declarator declarator: (array_declarator declarator: (field_identifier) @name))
  declarator: (field_identifier) @name
]) @definition.field_declaration

(field_declaration
  declarator: (_ [
    (field_identifier) @name
    (array_declarator declarator: (field_identifier) @name)
    (pointer_declarator declarator: (field_identifier) @name)
    (pointer_declarator declarator: (array_declarator declarator: (field_identifier) @name))
    declarator: (field_identifier) @name
  ]) @definition.field_declaration)

(field_declaration
  declarator: (_
    declarator: (_ [
      (field_identifier) @name
      (array_declarator declarator: (field_identifier) @name)
      (pointer_declarator declarator: (field_identifier) @name)
      (pointer_declarator declarator: (array_declarator declarator: (field_identifier) @name))
      declarator: (field_identifier) @name
    ]) @definition.field_declaration))


(enumerator
  name: (identifier) @name) @definition.d13

;;(declaration
;;  (init_declarator
;;    (pointer_declarator
;;      declarator: (identifier) @name))) @definition.identifier
;;

;; To exclude local variables declarations the `((translation_unit...` is used.
;; This does match only global declarations.

;; const uint8_t MY_CONSTANT = 20;
((translation_unit
   (declaration
    (init_declarator [
      declarator: (identifier) @name
      declarator: (qualified_identifier [
                    (identifier) @name
                    (qualified_identifier (identifier) @name)
                    (qualified_identifier (qualified_identifier (identifier) @name))
                    (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))))
                ]) @doc
            ])) @definition.d15))

;; const char MY_ARRAY[] = "Hello";
((translation_unit
(declaration
  (init_declarator
    (array_declarator [
      declarator: (identifier) @name
      declarator: (qualified_identifier [
                    (identifier) @name
                    (qualified_identifier (identifier) @name)
                    (qualified_identifier (qualified_identifier (identifier) @name))
                    (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))))
                ]) @doc
            ]))) @definition.d16))

;; extern uint16_t Test

;; to match storage class specifier (like "extern") do either
;;    (storage_class_specifier)
;; or
;;    (storage_class_specifier) @doc
;;    (#match? @doc "extern")
;; unfortunately cannot use custom capture like @_storage
;; therefore @doc is needed as workaround

(declaration
  (storage_class_specifier) [
    declarator: (identifier) @name
      declarator: (qualified_identifier [
                    (identifier) @name
                    (qualified_identifier (identifier) @name)
                    (qualified_identifier (qualified_identifier (identifier) @name))
                    (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))))
                ]) @doc
      ]) @definition.extern_declaration

(declaration
  (storage_class_specifier)
  (array_declarator[
    declarator: (identifier) @name
      declarator: (qualified_identifier [
                    (identifier) @name
                    (qualified_identifier (identifier) @name)
                    (qualified_identifier (qualified_identifier (identifier) @name))
                    (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))))
                ]) @doc
      ])) @definition.extern_array_declaration

(declaration
  (storage_class_specifier)
  (init_declarator[
    declarator: (identifier) @name
      declarator: (qualified_identifier [
                    (identifier) @name
                    (qualified_identifier (identifier) @name)
                    (qualified_identifier (qualified_identifier (identifier) @name))
                    (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))))
                ]) @doc
      ])) @definition.extern_init_declaration


;; References

;;(qualified_identifier
;;  name: (type_identifier) @name) @reference.r1

(call_expression [
  function: (identifier) @name
  function: (qualified_identifier [
                (identifier) @name
                (qualified_identifier (identifier) @name)
                (qualified_identifier (qualified_identifier (identifier) @name))
                (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))
                (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))
                (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))))
                (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))))
            ]) @doc
  ]) @reference.call_expression

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

;; &TemplateFunction<10,20>
(pointer_expression
    ([
        (template_function (identifier) @name)
        (qualified_identifier (template_function (identifier) @name))
        (qualified_identifier (qualified_identifier (template_function (identifier) @name)))
        (qualified_identifier (qualified_identifier (qualified_identifier (template_function (identifier) @name))))
        (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (template_function (identifier) @name)))))
        (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (template_function (identifier) @name))))))
        (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (template_function (identifier) @name)))))))
  ]) @doc) @reference.r55

;; XXX::Test()
(call_expression
  (qualified_identifier
    name: (identifier) @name)) @reference.r6

;; it would be nice to do post-processing and remove any @reference.argument_list.* (local variable used) which has no @definition.*
(argument_list [
  (identifier) @name
  (qualified_identifier [
           (identifier) @name
           (qualified_identifier (identifier) @name)
           (qualified_identifier (qualified_identifier (identifier) @name))
           (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))))
           ]) @doc
]) @reference.argument_list.identifier

;; this catches every namespace in the qualified identifier
(argument_list [
  (identifier) @name
  (qualified_identifier [
           (namespace_identifier) @name
           (qualified_identifier (namespace_identifier) @name)
           (qualified_identifier (qualified_identifier (namespace_identifier) @name))
           (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name)))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name)))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name))))))
         ]) @doc
]) @reference.argument_list.namespace_identifier



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

;; (declaration
;;   (init_declarator
;;     (argument_list
;;       (qualified_identifier) @name))) @reference.r14

(field_declaration [
  [(identifier) (type_identifier)] @name
  (qualified_identifier [
        [(identifier) (type_identifier)] @name
        (qualified_identifier [(identifier) (type_identifier)] @name)
        (qualified_identifier (qualified_identifier [(identifier) (type_identifier)] @name))
        (qualified_identifier (qualified_identifier (qualified_identifier [(identifier) (type_identifier)] @name)))
        (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier [(identifier) (type_identifier)] @name))))
        (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier [(identifier) (type_identifier)] @name)))))
        (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier [(identifier) (type_identifier)] @name))))))
  ]) @doc
]) @reference.r16a

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

(assignment_expression [
  right: (identifier) @name
  right: (qualified_identifier [
           (identifier) @name
           (qualified_identifier (identifier) @name)
           (qualified_identifier (qualified_identifier (identifier) @name))
           (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))))
           ]) @doc
]) @reference.assignment_expression.right.identifier

(assignment_expression [
  right: (identifier) @name
  right: (qualified_identifier [
           (namespace_identifier) @name
           (qualified_identifier (namespace_identifier) @name)
           (qualified_identifier (qualified_identifier (namespace_identifier) @name))
           (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name)))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name)))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name))))))
         ]) @doc
]) @reference.assignment_expression.right.namespace_identifier

(assignment_expression [
  left: (identifier) @name
  left: (qualified_identifier [
           (identifier) @name
           (qualified_identifier (identifier) @name)
           (qualified_identifier (qualified_identifier (identifier) @name))
           (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))))
           ]) @doc
]) @reference.assignment_expression.left.identifier

(assignment_expression [
  left: (identifier) @name
  left: (qualified_identifier [
           (namespace_identifier) @name
           (qualified_identifier (namespace_identifier) @name)
           (qualified_identifier (qualified_identifier (namespace_identifier) @name))
           (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name)))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name)))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name))))))
         ]) @doc
]) @reference.assignment_expression.left.namespace_identifier

;; this catches the identifier
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

;; this catches every namespace in the qualified identifier
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


(binary_expression [
  left: (identifier) @name
  left: (qualified_identifier [
           (identifier) @name
           (qualified_identifier (identifier) @name)
           (qualified_identifier (qualified_identifier (identifier) @name))
           (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))))
           ]) @doc
]) @reference.binary_expression.left.identifier

(binary_expression [
  left: (identifier) @name
  left: (qualified_identifier [
           (namespace_identifier) @name
           (qualified_identifier (namespace_identifier) @name)
           (qualified_identifier (qualified_identifier (namespace_identifier) @name))
           (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name)))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name)))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name))))))
         ]) @doc
])  @reference.binary_expression.left.namespace_identifier

(binary_expression [
  right: (identifier) @name
  right: (qualified_identifier [
           (identifier) @name
           (qualified_identifier (identifier) @name)
           (qualified_identifier (qualified_identifier (identifier) @name))
           (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name)))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (identifier) @name))))))
           ]) @doc
]) @reference.binary_expression.right.identifier

(binary_expression [
  right: (identifier) @name
  right: (qualified_identifier [
           (namespace_identifier) @name
           (qualified_identifier (namespace_identifier) @name)
           (qualified_identifier (qualified_identifier (namespace_identifier) @name))
           (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name)))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name)))))
           (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (namespace_identifier) @name))))))
         ]) @doc
])  @reference.binary_expression.right.namespace_identifier


;; XYZ_TYPE FunctionName(something) { ... }
(function_definition
  type: (type_identifier) @name) @reference.r27

(new_expression
  type: (type_identifier) @name) @reference.r28

(preproc_ifdef
  name: (identifier) @name) @reference.r29

(function_definition
  (function_declarator
    declarator: (identifier) @name)) @reference.function_definition_function_declarator

