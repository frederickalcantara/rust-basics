# Validating References with Lifetimes

Every reference in Rust has a **lifetime**, which is the scope for which the reference is valid. 

Most of the time, lifetimes are implicit and inferred, similar to how types are inferred. 

We must annotate types when multiple types are possible. 
Similarly, we must annotate lifetimes when the lifetime of references could be related in different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will be valid. 


## Preventing Dangling References with Lifetimes

The main goal of lifetimes is to prevent dangling references, which causes a program to reference data other than the data it's intended to reference. 