@0xcacdc66b16165af6;

struct Graph {
    vertices @0: List(Vertex);
}

struct Vertex {
    id @0: Int64;
}

struct Edge {
    fromVertex @0: Int64;
    toVertex @1: Int64;
}
