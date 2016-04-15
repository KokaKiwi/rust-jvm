rust-jvm
========

A basic experiment about implementing the JVM (Java SE 8) in Rust (http://www.rust-lang.org), maybe inspired by
[jvm.go](https://github.com/zxh0/jvm.go)

Current status: `WIP`

Documentation: http://kokakiwi.github.io/rust-jvm/jvm/index.html

JVM specification documentation: https://docs.oracle.com/javase/specs/jvms/se8/html/

Currently, the library can only parse almost all the Java .class file and print it.

The `rjvm` executable only take a class name, parse it and print it.

TO-DO List
----------

- [ ] Read `*.class` files
  - [ ] Read attributes
    - [ ] Read `BootstrapMethods` attribute
    - [ ] Read `RuntimeVisibleParameterAnnotations` attribute
    - [ ] Read `RuntimeInvisibleParameterAnnotations` attribute
    - [ ] Read `AnnotationDefault` attribute
    - [ ] Read `MethodParameters` attribute
    - [ ] Read `LocalVariableTypeTable` attribute
    - [ ] Read `RuntimeInvisibleAnnotations` attribute
    - [ ] Read `RuntimeVisibleTypeAnnotations` attribute
    - [ ] Read `RuntimeInvisibleTypeAnnotations` attribute
    - [ ] Read `StackMapTable` attribute
      - [ ] Read `StackMapFrame` struct
      - [ ] Read `VerificationTypeInfo` struct
- [ ] Implement classpath structs
