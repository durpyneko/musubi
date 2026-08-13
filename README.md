<div align="center"> <img src=".github/imgs/musubi-rounded.png" alt="Musubi logo" width="300">
<br>
<h1>Musubi (結び)</h1>

<p>
    <b>Musubi (結び)</b> refers to tying, binding, joining, or forming a connection 
    <br>
    and is based on <a href="https://en.wikipedia.org/wiki/Directed_acyclic_graph">
        Directed Acyclic Graphs (DAGs)
    </a>
    
</p>

<pre>
A ──┐
    ├── C ──┐
B ──┘       │
            ├── Result
D ──────────┘
</pre>

<p>
    
</p>

</div>

What is Musubi?

• Musubi is a Rust-based graph composition system for building workflows where multiple independent inputs can be transformed, combined, and ultimately resolved into a single result.

• Rather than representing a process as a simple linear pipeline, Musubi represents it as a graph of interconnected nodes.

• This allows independent parts of a workflow to be processed separately before being joined together. 

• Musubi also allows super fast backwords traversal!