import { createCanvas } from "canvas"

interface Crystal {
    x: number;
    y: number;
    color: number;
}

const wNegX = 0.1;
const wNegY = 0.4;
const wPosX = 0.1;
const wPosY = 0.1;

// Weights for diagonal directions
const wNegXNegY = 0.05; // top-left
const wNegXPosY = 0.05; // bottom-left
const wPosXNegY = 0.05; // top-right
const wPosXPosY = 0.05; // bottom-right

const startCrystals = 50;
const pNewCrystal = 0.001; // Probability of placing a crystal in a new position

const width = 256;
const height = 256;
const canvas = createCanvas(width, height);
const ctx = canvas.getContext("2d");

function coordsToIndex(x: number, y: number): number {
    return y * width + x;
}

function indexToCoords(index: number): { x: number; y: number } {
    return {
        x: index % width,
        y: Math.floor(index / width)
    };
}


// Initialize data array with nulls in a single operation
const data: (Crystal | null)[] = Array(width * height).fill(null);

// Use a Set to track occupied positions for faster lookups
const occupiedPositions = new Set<number>();
const growingPoints: Crystal[] = [];

// Initialize crystals more efficiently
for (let i = 0; i < startCrystals; i++) {
    let x, y, index;

    // Ensure we don't place crystals on already occupied positions
    do {
        x = Math.floor(Math.random() * width);
        y = Math.floor(Math.random() * height);
        index = coordsToIndex(x, y);
    } while (occupiedPositions.has(index));

    const color = Math.random() * 255;
    const crystal: Crystal = { x, y, color };

    data[index] = crystal;
    occupiedPositions.add(index);
    growingPoints.push(crystal);
}

// More efficient processing of growing points
// Use a queue data structure for better performance
const queue = [...growingPoints];
growingPoints.length = 0; // Clear the original array

// Precompute direction weights and offsets for faster neighbor calculations
const directions = [
    { dx: -1, dy: 0, w: wNegX }, // left
    { dx: 1, dy: 0, w: wPosX },  // right
    { dx: 0, dy: -1, w: wPosY }, // up
    { dx: 0, dy: 1, w: wNegY },  // down
    // Diagonal directions
    { dx: -1, dy: -1, w: wNegXNegY }, // top-left
    { dx: -1, dy: 1, w: wNegXPosY },  // bottom-left
    { dx: 1, dy: -1, w: wPosXNegY },  // top-right
    { dx: 1, dy: 1, w: wPosXPosY }    // bottom-right
];

while (queue.length > 0 || data.some(crystal => crystal === null)) {
    if (Math.random() < pNewCrystal) {
        for (let i = 0; i < 100; i++) {
            // Randomly select a crystal from the queue
            const randomIndex = Math.floor(Math.random() * data.length);
            if (occupiedPositions.has(randomIndex))
                continue

            const { x: nX, y: nY } = indexToCoords(randomIndex)
            const newCrystal: Crystal = {
                color: Math.random() * 255,
                x: nX,
                y: nY
            }

            // Update data structure
            const index = coordsToIndex(nX, nY);
            data[index] = newCrystal;
            occupiedPositions.add(index);
            queue.push(newCrystal);
            break
        }

        continue;
    }

    // Faster removal from beginning of array instead of random splicing
    const crystal = queue.shift()!;
    const { x, y, color } = crystal;

    // Shuffle directions once per crystal
    const shuffledDirs = [...directions].sort(() => Math.random() - 0.5);

    let hasFound = false;
    let hasSkipped = false;

    for (const { dx, dy, w } of shuffledDirs) {
        const nX = x + dx;
        const nY = y + dy;

        // Boundary check
        if (nX < 0 || nX >= width || nY < 0 || nY >= height) {
            continue;
        }

        const index = coordsToIndex(nX, nY);

        // Use Set for faster occupied position checking
        if (occupiedPositions.has(index)) {
            continue;
        }

        if (Math.random() > w) {
            hasSkipped = true;
            continue;
        }

        // Create new crystal
        const newCrystal: Crystal = { x: nX, y: nY, color };

        // Update data structures efficiently
        data[index] = newCrystal;
        occupiedPositions.add(index);
        queue.push(crystal);   // Add original crystal back to queue
        queue.push(newCrystal); // Add new crystal to queue

        hasFound = true;
        break;
    }

    if (!hasFound && hasSkipped) {
        queue.push(crystal);
    }
}



// Create image data once
const imgData = ctx.createImageData(width, height);
const buffer = imgData.data;

// Pre-calculate pixel data values for empty spaces
const emptyR = 255, emptyG = 0, emptyB = 0, alpha = 255;

// Fill with default color first (more efficient than checking each pixel)
for (let i = 0; i < width * height; i++) {
    const offset = i * 4;
    buffer[offset] = emptyR;     // R
    buffer[offset + 1] = emptyG; // G
    buffer[offset + 2] = emptyB; // B
    buffer[offset + 3] = alpha;  // A
}

// Only process occupied positions
for (const index of occupiedPositions) {
    const crystal = data[index]!;
    const color = crystal.color;
    const shifted = Math.min(255, color + 10);

    const offset = index * 4;
    buffer[offset] = color;     // R
    buffer[offset + 1] = shifted; // G
    buffer[offset + 2] = shifted; // B
    // Alpha already set to 255
}

ctx.putImageData(imgData, 0, 0);
Bun.write("output.png", canvas.toBuffer("image/png"));