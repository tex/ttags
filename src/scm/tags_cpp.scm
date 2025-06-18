;; Order Matters

;; Definitions

(struct_specifier
  name: (type_identifier) @name
  body: (_)
) @definition.d1

(union_specifier
  name: (type_identifier) @name) @definition.union_specifier

(template_declaration
  (function_definition
    (function_declarator
      declarator: [
        (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (template_function name: (_) @name)))))))
        (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (template_function name: (_) @name))))))
        (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (template_function name: (_) @name)))))
        (qualified_identifier (qualified_identifier (qualified_identifier (template_function name: (_) @name))))
        (qualified_identifier (qualified_identifier (template_function name: (_) @name)))
        (qualified_identifier (template_function name: (_) @name))
        (template_function name: (_) @name)
  ]) @doc
) @definition.template_declaration.function_definition.function_declarator.declarator.template_function)

(template_declaration
  (function_definition
    declarator: (function_declarator
      declarator: (identifier) @name))
) @definition.template_declaration.function_definition.function_declarator.declarator.identifier


(function_definition
  (function_declarator [
    declarator: (qualified_identifier [
                  (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier name: (_) @name))))))
                  (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier name: (_) @name)))))
                  (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier name: (_) @name))))
                  (qualified_identifier (qualified_identifier (qualified_identifier name: (_) @name)))
                  (qualified_identifier (qualified_identifier name: (_) @name))
                  (qualified_identifier name: (_) @name)
                  name: (_) @name
    declarator: (_) @name
    ]) @doc
  ]) @definition.function_definition.function_declarator.declarator)

(function_definition (_
  (function_declarator [
      declarator: (_) @name
      declarator: (qualified_identifier [
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier name: (_) @name))))))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier name: (_) @name)))))
                    (qualified_identifier (qualified_identifier (qualified_identifier (qualified_identifier name: (_) @name))))
                    (qualified_identifier (qualified_identifier (qualified_identifier name: (_) @name)))
                    (qualified_identifier (qualified_identifier name: (_) @name))
                    (qualified_identifier name: (_) @name)
                    name: (_) @name
             ]) @doc
  ]) @definition.function_definition._.function_declarator.declarator))

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

;; uint32_t FunctionDeclaration(uint32_t param);
(declaration
  (function_declarator
    declarator: (identifier) @name)) @definition.declaration.function_declarator.identifier

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
]) @definition.field_declaration.field_identifier

(field_declaration
  declarator: (_ [
    (field_identifier) @name
    (array_declarator declarator: (field_identifier) @name)
    (pointer_declarator declarator: (field_identifier) @name)
    (pointer_declarator declarator: (array_declarator declarator: (field_identifier) @name))
    declarator: (field_identifier) @name
  ]) @definition.field_declaration._.field_identifier)

(field_declaration
  declarator: (_
    declarator: (_ [
      (field_identifier) @name
      (array_declarator declarator: (field_identifier) @name)
      (pointer_declarator declarator: (field_identifier) @name)
      (pointer_declarator declarator: (array_declarator declarator: (field_identifier) @name))
      declarator: (field_identifier) @name
    ]) @definition.field_declaration._._.field_identifier))


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
            ])) @definition.translation_unit.init_declarator))

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
            ]))) @definition.translation_unit.init_declarator.array_declarator))

(namespace_definition body: (_
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
            ])) @definition.namespace_definition.init_declarator))

;; const char MY_ARRAY[] = "Hello";
(namespace_definition body: (_
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
            ]))) @definition.namespace_definition.init_declarator.array_declarator))


;; extern uint16_t Test

;; to match storage class specifier (like "extern") do either
;;    (storage_class_specifier)
;; or
;;    (storage_class_specifier) @doc
;;    (#match? @doc "extern")
;; unfortunately cannot use custom capture like @_storage
;; therefore @doc is needed as workaround

(declaration
  (storage_class_specifier) [ ;; extern or static...
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
      ]) @definition.storage_class_specifier.declaration

(declaration
  (storage_class_specifier) ;; extern or static...
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
      ])) @definition.storage_class_specifier.declaration.array_declarator

(declaration
  (storage_class_specifier) ;; extern or static...
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
      ])) @definition.storage_class_specifier.declaration.init_declarator

(declaration
  (storage_class_specifier) ;; extern or static...
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
            ]))) @definition.storage_class_specifier.declaration.init_declarator.array_declarator

;; References

  (identifier) @name @reference.identifier
  (field_identifier) @name @reference.field_identifier
  (namespace_identifier) @name @reference.namespace_identifier
  (type_identifier) @name @reference.type_identifier

