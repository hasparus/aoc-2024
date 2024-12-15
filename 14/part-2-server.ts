import { type BoardSize, type RobotState, parseInput } from "./index";

const WIDTH = 101;
const HEIGHT = 103;

const UNIQUE_BOARDS = WIDTH * HEIGHT;
const MAX_ENTROPY = 0.265;

function createHtmlBoard(robots: RobotState[], { width, height }: BoardSize) {
  return `
    <!DOCTYPE html>
    <html>
      <head>
        <title>Robot Positions Timeline</title>
        <style>
          body {
            font-family: monospace;
            background: #f0f0f0;
            margin: 0;
            padding: 16px;
          }
          #boards {
            display: flex;
            flex-wrap: wrap;
            gap: 4px;
            align-items: flex-start;
          }
          .board-container {
            position: relative;
          }
          .board-container:hover::after {
            content: attr(title);
            position: absolute;
            left: 0;
            top: 100%;
            background: black;
            color: white;
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 12px;
            white-space: nowrap;
            z-index: 1;
          }
          .board {
            image-rendering: pixelated;
            display: block;
            border: 1px solid #ddd;
            background: white;
          }
        </style>
      </head>
      <body>
        <div id="boards"></div>
        <script>
          const width = ${width};
          const height = ${height};
          const robots = ${JSON.stringify(robots)};

          function calculateEntropy(positions) {
            const totalCells = width * height;
            let occupiedCount = 0;
            for (let y = 0; y < height; y++) {
              for (let x = 0; x < width; x++) {
                if (positions[y][x] > 0) occupiedCount++;
              }
            }
            
            const p1 = occupiedCount / totalCells;
            const p2 = 1 - p1;
            
            const e1 = p1 === 0 ? 0 : -p1 * Math.log2(p1);
            const e2 = p2 === 0 ? 0 : -p2 * Math.log2(p2);
            
            return e1 + e2;
          }

          function renderBoard(canvas, positions) {
            const ctx = canvas.getContext('2d');
            const imageData = ctx.createImageData(width, height);
            const data = imageData.data;
            const midX = Math.floor(width / 2);
            const midY = Math.floor(height / 2);

            for (let y = 0; y < height; y++) {
              for (let x = 0; x < width; x++) {
                const i = (y * width + x) * 4;
                const isMiddle = x === midX || y === midY;
                const hasRobot = positions[y][x] > 0;

                if (hasRobot) {
                  if (isMiddle) {
                    data[i] = 255;     // R
                    data[i + 1] = 0;   // G
                    data[i + 2] = 0;   // B
                  } else {
                    data[i] = 0;       // R
                    data[i + 1] = 0;   // G
                    data[i + 2] = 0;   // B
                  }
                  data[i + 3] = 255;   // A
                } else if (isMiddle) {
                  data[i] = 255;       // R
                  data[i + 1] = 204;   // G
                  data[i + 2] = 204;   // B
                  data[i + 3] = 255;   // A
                } else {
                  data[i + 3] = 0;     // A
                }
              }
            }
            ctx.putImageData(imageData, 0, 0);
          }

          function processTime(time) {
            const positions = Array.from({ length: height }, () => new Array(width).fill(0));
            const currentRobots = JSON.parse(JSON.stringify(robots));
            
            for (let t = 0; t < time; t++) {
              for (const robot of currentRobots) {
                robot.position.x = (robot.position.x + robot.velocity.x + width) % width;
                robot.position.y = (robot.position.y + robot.velocity.y + height) % height;
              }
            }

            for (const robot of currentRobots) {
              positions[robot.position.y][robot.position.x]++;
            }

            return positions;
          }

          function createBoard(time) {
            const positions = processTime(time);
            const entropy = calculateEntropy(positions);
            
            if (entropy > ${MAX_ENTROPY}) return null;

            const container = document.createElement('div');
            container.className = 'board-container';
            container.title = \`t=\${time} entropy=\${entropy.toFixed(3)}\`;
            
            const canvas = document.createElement('canvas');
            canvas.width = width;
            canvas.height = height;
            canvas.className = 'board';
            
            container.appendChild(canvas);
            renderBoard(canvas, positions);
            return container;
          }

          let currentTime = 0;
          const container = document.getElementById('boards');

          function addMoreBoards() {
            const fragment = document.createDocumentFragment();
            let added = 0;
            
            while (added < 10 && currentTime < ${UNIQUE_BOARDS}) {
              const board = createBoard(currentTime++);
              if (board) {
                fragment.appendChild(board);
                added++;
              }
            }
            
            container.appendChild(fragment);
            
            if (currentTime < ${UNIQUE_BOARDS}) {
              requestAnimationFrame(addMoreBoards);
            }
          }

          addMoreBoards();
        </script>
      </body>
    </html>
  `;
}

const server = Bun.serve({
  port: 3000,
  async fetch(req) {
    const inputFile = Bun.file("./input.txt");
    const input = await inputFile.text();
    const robots = parseInput(input);

    const html = createHtmlBoard(robots, { width: WIDTH, height: HEIGHT });

    return new Response(html, {
      headers: {
        "Content-Type": "text/html",
      },
    });
  },
});

console.log(`Server running at ${server.url}`);
