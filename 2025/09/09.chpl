use IO;
use IO.FormattedIO;
use List;
use LinkedLists;
use LinearAlgebra;
use Set;

record Point {
    const x, y: real;
}

const NilPoint = new Point(inf, inf);

record Segment {
    const p1, p2: Point;
}

record Line {
    const a, b, c;
}

record Rectangle {
    const p1, p2: Point;
}

inline proc line(l: Segment) {
    const a = l.p1.y - l.p2.y;
    const b = l.p2.x - l.p1.x;
    const c = -a * l.p1.x - b * l.p1.y;
    var norm = sqrt(a * a + b * b);
    if abs(norm) < 0.0001 then norm = 1.0;
    return new Line(a/norm, b/norm, c/norm);
}

proc readInput(): []Point {
    var points: LinkedList(Point);
    while true {
        var x, y: int;
        if !readf("%i,%i", x, y) then break;
        points.push_back(new Point(x, y));
    }
    return points.these();
}

inline proc rectArea(r: Rectangle): real {
    return (abs(r.p2.y - r.p1.y) + 1) * (abs(r.p2.x - r.p1.x) + 1);
}

inline proc isBetween(x, n1, n2) {
    return abs(x - n1) <= abs(n1 - n2) && abs(x - n2) <= abs(n1 - n2);
}

// https://cp-algorithms.com/geometry/segments-intersection.html
// https://en.wikipedia.org/wiki/Line–line_intersection
inline proc intersectionPoint(l1: Segment, l2: Segment): Point {

    const line1 = line(l1);
    const line2 = line(l2);

    const angle1 = line1.a / line1.b;
    const angle2 = line2.a / line2.b;

    if abs(angle1 - angle2) < 0.0001 {
        // dont care for parallel Lines
        return NilPoint;
    }

    // (a1x + b1y + c1) = 0
    // -y = (a1x + c1) / b1 = (a2x + c2) / b2
    // b2a1x + b2c1 = b1a2x + b1c2
    // b2a1x - b1a2x = b1c2 - b2c1
    // x(b2a1 - b1a2) = b1c2 - b2c1
    // x = (b1c2 - b2c1) / (b2a1 - b1a2)
    const x = (line1.b * line2.c - line2.b * line1.c) / (line2.b * line1.a - line1.b * line2.a);
    // -y = (a1x + c1) / b1
    // y = -(a1x + c1) / b1
    const y = -(line1.a * x + line1.c) / line1.b;

    // check if intersection not happens inside the segments
    if ! (isBetween(x, l1.p1.x, l1.p2.x) &&
            isBetween(x, l2.p1.x, l2.p2.x) &&
            isBetween(y, l1.p1.y, l1.p2.y) &&
            isBetween(y, l2.p1.y, l2.p2.y)) {
        return NilPoint;
    }

    return new Point(x, y);
}


inline proc pointInsideRect(point: Point, rect: Rectangle): bool {
    const left = min(rect.p1.x, rect.p2.x);
    const right = max(rect.p1.x, rect.p2.x);
    const top = min(rect.p1.y, rect.p2.y);
    const  bottom = max(rect.p1.y, rect.p2.y);
    const intersects = point.x >= left && point.x <= right && point.y >= top && point.y <= bottom;
    const in_border = (point.x == left || point.x == right) || (point.y == top || point.y == bottom);
    return intersects && !in_border;
}

proc allIntersections(points: [] Point): [] Point {
    var intersections: set(Point);
    for p1 in points {
        for p2 in points {
            for i in points.domain {
                const p3 = points[i];
                const p4 = points[(i+1) % points.size];
                const intersection = intersectionPoint(new Segment(p1, p2), new Segment(p3, p4));
                if intersection != NilPoint {
                    intersections.add(intersection);
                }
            }
        }
    }

    return intersections.these();
}

proc main() {
    const points = readInput();

    var maxArea = -inf;
    for p1 in points {
        for p2 in points {
            const curArea = rectArea(new Rectangle(p1, p2));
            maxArea = max(maxArea, curArea);
        }
    }

    writef("Max Rectangle Area: %i\n", maxArea);

    const intersections = allIntersections(points);

    maxArea = -inf;
    for i1 in points.domain {
        for i2 in points.domain[i1+1..] {
            const rect = new Rectangle(points[i1], points[i2]);
            const area = rectArea(rect);
            if area <= maxArea then continue;
            var intersectionInside = false;
            for p3 in intersections {
                if pointInsideRect(p3, rect) {
                    intersectionInside = true;
                    break;
                }
            }
            if !intersectionInside then maxArea = area;
        }
    }

    writef("Max Rectangle Inside Valid Area: %i\n", maxArea);
}
