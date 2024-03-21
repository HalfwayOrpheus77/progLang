// First, I hence shall define such that a rectangle exists
class Rectangle {
    constructor(origin, width, height) {
        this.origin = origin;
        this.width = width;
        this.height = height;
    }

    // Time to compute the perimeter of this rectangle!
    perimeter() {
        return 2 * (this.width + this.height);
    }

    // The area of this rectangle may be big!
    area() {
        return this.width * this.height;
    }

    // Four corners. Of a rectangle. Lableded.
    corners() {
        let bottomLeft = this.origin;
        let topLeft = {x: this.origin.x, y: this.origin.y + this.height};
        let bottomRight = {x: this.origin.x + this.width, y: this.origin.y};
        let topRight = {x: this.origin.x + this.width, y: this.origin.y + this.height};

        return [bottomLeft, topLeft, bottomRight, topRight];
    }
}

// To be or not to be. TEST
let rect = new Rectangle({x: 0, y: 0}, 5, 10);
console.log("Perimeter: " + rect.perimeter());
console.log("Area: " + rect.area());
console.log("Corners: ", rect.corners());
