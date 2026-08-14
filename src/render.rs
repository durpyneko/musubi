use crate::graph::Graph;
use crate::layout::PositionedNode;

struct Canvas {
    width: usize,
    height: usize,
    cells: Vec<Vec<char>>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![vec![' '; width]; height],
        }
    }

    fn set(&mut self, x: usize, y: usize, c: char) {
        if x < self.width && y < self.height {
            self.cells[y][x] = c;
        }
    }

    fn text(&mut self, x: usize, y: usize, text: &str) {
        for (offset, c) in text.chars().enumerate() {
            self.set(x + offset, y, c);
        }
    }

    fn print(&self) {
        for row in &self.cells {
            println!("{}", row.iter().collect::<String>().trim_end());
        }
    }
}

fn position_of(positions: &[PositionedNode], node_id: usize) -> Option<&PositionedNode> {
    positions.iter().find(|position| position.node == node_id)
}

fn column_widths(graph: &Graph, positions: &[PositionedNode]) -> Vec<usize> {
    let max_x = positions
        .iter()
        .map(|position| position.x)
        .max()
        .unwrap_or(0);

    let mut widths = vec![0; max_x + 1];

    for position in positions {
        let name = graph.node_display_name(position.node);

        widths[position.x] = widths[position.x].max(name.len());
    }

    widths
}

fn column_x(widths: &[usize], column: usize) -> usize {
    widths.iter().take(column).sum::<usize>() + column * 4
}

fn draw_horizontal(canvas: &mut Canvas, from_x: usize, to_x: usize, y: usize) {
    if to_x <= from_x {
        return;
    }

    for x in (from_x + 1)..to_x {
        canvas.set(x, y, '─');
    }
}

fn draw_edge(
    canvas: &mut Canvas,
    graph: &Graph,
    positions: &[PositionedNode],
    from_id: usize,
    to_id: usize,
    widths: &[usize],
) {
    let from = match position_of(positions, from_id) {
        Some(position) => position,
        None => return,
    };

    let to = match position_of(positions, to_id) {
        Some(position) => position,
        None => return,
    };

    let from_name = graph.node_display_name(from_id);
    let to_x = column_x(widths, to.x);

    let from_x = column_x(widths, from.x);
    let from_right = from_x + from_name.len();

    let from_y = from.y;
    let to_y = to.y;

    // The vertical connector sits immediately before the destination.
    //
    // Example:
    //
    // Apple ──┐
    //         ├──Candy Apple
    // Candy ──┘
    //
    let connector_x = to_x.saturating_sub(2);

    // Simple connection where both nodes are on the same row.
    if from_y == to_y {
        draw_horizontal(canvas, from_right, to_x, from_y);
        return;
    }

    // Horizontal line from the source node to the vertical connector.
    draw_horizontal(canvas, from_right, connector_x, from_y);

    // Vertical portion of the connection.
    let min_y = from_y.min(to_y);
    let max_y = from_y.max(to_y);

    for y in min_y..=max_y {
        canvas.set(connector_x, y, '│');
    }

    // Replace the vertical line's endpoints with corners.
    if from_y < to_y {
        // Source is above destination.
        canvas.set(connector_x, from_y, '┐');
        canvas.set(connector_x, to_y, '├');
    } else {
        // Source is below destination.
        canvas.set(connector_x, from_y, '┘');
        canvas.set(connector_x, to_y, '├');
    }

    // Connect the merge point to the destination node.
    draw_horizontal(canvas, connector_x, to_x, to_y);
}

pub fn render(graph: &Graph, positions: &[PositionedNode]) {
    if positions.is_empty() {
        return;
    }

    let widths = column_widths(graph, positions);

    // Calculate canvas dimensions.
    let max_x = positions
        .iter()
        .map(|position| position.x)
        .max()
        .unwrap_or(0);

    let max_y = positions
        .iter()
        .map(|position| position.y)
        .max()
        .unwrap_or(0);

    let last_column_x = column_x(&widths, max_x);
    let last_column_width = widths[max_x];

    let width = last_column_x + last_column_width + 2;
    let height = max_y + 1;

    let mut canvas = Canvas::new(width, height);

    // ------------------------------------------------------------
    // Draw edges first.
    //
    // Nodes are drawn afterwards so that node names always appear
    // on top of connection lines.
    // ------------------------------------------------------------

    for edge in graph.edges() {
        draw_edge(&mut canvas, graph, positions, edge.from, edge.to, &widths);
    }

    // Draw nodes
    for position in positions {
        let x = column_x(&widths, position.x);
        let y = position.y;

        let name = graph.node_display_name(position.node);

        canvas.text(x, y, &name);
    }

    canvas.print();
}
