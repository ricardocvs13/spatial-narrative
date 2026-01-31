# Graph Export & Visualization

Export your narrative graphs for visualization in external tools.

## DOT Format (Graphviz)

Export to DOT format for rendering with Graphviz.

### Basic Export

```rust
use spatial_narrative::graph::NarrativeGraph;

let graph = NarrativeGraph::from_events(events);
graph.connect_temporal();

// Export to DOT
let dot = graph.to_dot();

// Save to file
std::fs::write("narrative.dot", &dot)?;
```

### Rendering with Graphviz

```bash
# Install Graphviz
# Ubuntu/Debian: sudo apt install graphviz
# macOS: brew install graphviz
# Windows: choco install graphviz

# Render to PNG
dot -Tpng narrative.dot -o narrative.png

# Render to SVG (better for web)
dot -Tsvg narrative.dot -o narrative.svg

# Render to PDF
dot -Tpdf narrative.dot -o narrative.pdf
```

### Custom Options

```rust
use spatial_narrative::graph::DotOptions;

// Timeline layout (left-to-right)
let timeline_dot = graph.to_dot_with_options(DotOptions::timeline());

// Hierarchical layout (top-to-bottom)
let hier_dot = graph.to_dot_with_options(DotOptions::hierarchical());

// Custom options
let options = DotOptions {
    rank_direction: "LR".to_string(),
    node_shape: "ellipse".to_string(),
    font_name: "Helvetica".to_string(),
};
let custom_dot = graph.to_dot_with_options(options);
```

### Online Visualization

Paste DOT output into online tools:
- [Graphviz Online](https://dreampuf.github.io/GraphvizOnline/)
- [Edotor](https://edotor.net/)
- [Viz.js](http://viz-js.com/)

## JSON Format

Export for web visualization libraries.

### Basic Export

```rust
// Compact JSON
let json = graph.to_json();

// Pretty-printed JSON
let json = graph.to_json_pretty();

// Save to file
std::fs::write("narrative.json", &json)?;
```

### JSON Structure

```json
{
  "nodes": [
    {
      "id": 0,
      "event_id": "550e8400-e29b-41d4-a716-446655440000",
      "text": "Event description",
      "location": { "lat": 40.7128, "lon": -74.006 },
      "timestamp": "2024-01-15T10:00:00+00:00",
      "tags": ["conference", "technology"]
    }
  ],
  "edges": [
    {
      "source": 0,
      "target": 1,
      "type": "Temporal",
      "weight": 1.0,
      "label": null
    }
  ],
  "metadata": {
    "node_count": 5,
    "edge_count": 8
  }
}
```

### Web Visualization Libraries

#### D3.js

```javascript
// Load the JSON
fetch('narrative.json')
  .then(response => response.json())
  .then(data => {
    // Create force-directed graph
    const simulation = d3.forceSimulation(data.nodes)
      .force("link", d3.forceLink(data.edges).id(d => d.id))
      .force("charge", d3.forceManyBody())
      .force("center", d3.forceCenter(width / 2, height / 2));
  });
```

#### Cytoscape.js

```javascript
const cy = cytoscape({
  container: document.getElementById('cy'),
  elements: {
    nodes: data.nodes.map(n => ({ data: { id: n.id, label: n.text } })),
    edges: data.edges.map(e => ({ data: { source: e.source, target: e.target } }))
  }
});
```

#### Sigma.js

```javascript
const graph = new graphology.Graph();
data.nodes.forEach(n => graph.addNode(n.id, { label: n.text }));
data.edges.forEach(e => graph.addEdge(e.source, e.target));

const sigma = new Sigma(graph, container);
```

## Node Colors

The DOT export uses automatic node coloring:

| Color | Meaning |
|-------|---------|
| 🟢 Light Green | Root nodes (no incoming edges) |
| 🩷 Light Pink | Leaf nodes (no outgoing edges) |
| 🔵 Light Blue | Hub nodes (high connectivity) |
| 🟡 Light Yellow | Regular nodes |

## Edge Styles

Edges are styled by type:

| Type | Color | Style |
|------|-------|-------|
| Temporal | Blue | Solid |
| Spatial | Magenta | Dashed |
| Causal | Orange | Bold |
| Thematic | Red | Dotted |
| Reference | Olive | Solid |
| Custom | Gray | Solid |
