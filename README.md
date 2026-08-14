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
<p> 
</p>
</div>

<pre>
A ──┐
    ├── C ──┐
B ──┘       │
            ├── Result
D ──────────┘
</pre>

# What is Musubi?

• Musubi is a Rust-based graph composition system for building workflows where multiple independent inputs can be transformed, combined, and ultimately resolved into a single result.

• Rather than representing a process as a simple linear pipeline, Musubi represents it as a graph of interconnected nodes.

• This allows independent parts of a workflow to be processed separately before being joined together. 

• Musubi also allows super fast backwords traversal and instant node resolution!

# Example(s)

## 1: Simple

<pre>
Candy ─┐
       ├─Candy Apple ─┐
Apple ─┘              ├─Super Candy Apple
                      │
Super ────────────────┘
</pre>

<details>
<summary>View source</summary>

```rs
use musubi::graph::Graph;
use musubi::layout::layout;
use musubi::node::Node;
use musubi::render::render;

fn main() {
    let mut graph = Graph::new();

    let b = graph.add_node(Node::new(Some("Candy".into())));
    let a = graph.add_node(Node::new(Some("Apple".into())));
    let c = graph.add_node(Node::new(None));

    let d = graph.add_node(Node::new(Some("Super".into())));
    let e = graph.add_node(Node::new(None));

    graph.connect(a, c, "".into());
    graph.connect(b, c, "".into());

    graph.connect(c, e, "".into());
    graph.connect(d, e, "".into());

    let positions = layout(&graph);

    render(&graph, &positions);
}
```
</details>

## 2: Little Complex

<pre>
Candy ─┐
       ├─Candy Apple ─┐
Apple ─┘              │
                      ├─Super Big Candy Apple
Super ─┐              │
       ├─Super Big ───┘
Big ───┘
</pre>

<details>
<summary>View source</summary>

```rs
use musubi::graph::Graph;
use musubi::layout::layout;
use musubi::node::Node;
use musubi::render::render;

fn main() {
    let mut graph = Graph::new();

    let b = graph.add_node(Node::new(Some("Candy".into())));
    let a = graph.add_node(Node::new(Some("Apple".into())));
    let c = graph.add_node(Node::new(None));

    let e = graph.add_node(Node::new(Some("Super".into())));
    let d = graph.add_node(Node::new(Some("Big".into())));
    let f = graph.add_node(Node::new(None));
    let g = graph.add_node(Node::new(None));

    graph.connect(a, c, "".into());
    graph.connect(b, c, "".into());

    graph.connect(d, f, "".into());
    graph.connect(e, f, "".into());

    graph.connect(c, g, "".into());
    graph.connect(f, g, "".into());

    let positions = layout(&graph);

    render(&graph, &positions);
}
```
</details>

# Roadmap

- [x] Render a Graph
- [ ] Append a method to a Node
- [ ] Traverse the Graph

# Status

🚧 Early development — API and architecture are subject to change.
