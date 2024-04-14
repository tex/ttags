;; What follows is a "manifest" equivalent to the command line you gave.
;; You can store it in a file that you may then pass to any 'guix' command
;; that accepts a '--manifest' (or '-m') option.

;; use 'samply record target/debug/ttags' to performance analysis

(specifications->manifest
  (list "rust"
        "rust:cargo"
        "clang-toolchain"
        "rust-analyzer"
        "perf"))

