const IMG_WIDTH = 1600;
const IMG_HEIGHT = 800;
const IMG_HALF_WIDTH = IMG_WIDTH / 2;
const IMG_HALF_HEIGHT = IMG_HEIGHT / 2;
const IMG_WIDTH_SCALE = IMG_WIDTH / 360;
const IMG_HEIGHT_SCALE = IMG_HEIGHT / 180;
const PI2 = Math.PI * 2;
const ANGLE = 45 * Math.PI / 180;

const USER_SQUARE_COLOR = "red";
const ISP_SQUARE_COLOR = "orange";

let userSquareXY = null;
let ispSquareXY = null;

function parseCoordinates(str) {
  const [lat_str, lon_str] = str.split('|');
  return [parseFloat(lat_str), parseFloat(lon_str)];
}
async function drawMap() {
  let canvas = document.getElementById("canvas");
  if (canvas.getContext) {
    const ctx = canvas.getContext("2d");
    const img = new Image;
    img.src = "img/map.jpeg";
    img.onload = async() => {
      ctx.drawImage(img, 0, 0);
      const response = await fetch(window.location.origin + "/locations");
      const reader = response.body.getReader();
      const decoder = new TextDecoder();
      while (true) {
        const {done, value} = await reader.read();
        if (done) break;
        const str = decoder.decode(value);
        const iter = str.split('\n');
        iter.pop();
        for (const line of iter) {
          const [latitude, longitude] = parseCoordinates(line);
          if (isNaN(latitude) || isNaN(longitude)) {
            continue;
          }

          const circle = new Path2D;
          circle.arc(
              IMG_HALF_WIDTH + IMG_WIDTH_SCALE * longitude,
              IMG_HEIGHT - (IMG_HALF_HEIGHT + IMG_HEIGHT_SCALE * latitude),
              1,
              0,
              PI2
          );
          ctx.fillStyle = "#fefff8";
          ctx.fill(circle);
        }
      }
    }
  } else {
    console.error("Couldn't get canvas context");
  }
}

function drawLocationSquare(ctx, color) {
  const size = 15;
  ctx.strokeStyle = color;
  ctx.lineWidth = 15;
  ctx.beginPath();
  ctx.moveTo(-size / 2, 0);
  ctx.lineTo(size / 2, 0);
  ctx.moveTo(0, -size / 2);
  ctx.lineTo(0, size / 2);
  ctx.stroke();
}

function drawCurrentLocation(json, color) {
  let canvas = document.getElementById("canvas");
  if (canvas.getContext) {
    const ctx = canvas.getContext("2d");
    const status = json["status"];
    const resp = json["response"];
    if (status === "ok" && typeof resp !== "undefined") {
      const [latitude, longitude] = parseCoordinates(resp);
      if (isNaN(latitude) || isNaN(longitude)) {
        return;
      }

      // save default rotation
      ctx.save();
      const x = IMG_HALF_WIDTH + IMG_WIDTH_SCALE * longitude;
      const y = IMG_HEIGHT - (IMG_HALF_HEIGHT + IMG_HEIGHT_SCALE * latitude);
      ctx.translate(x, y);
      ctx.rotate(ANGLE);
      if (color === USER_SQUARE_COLOR) {
        userSquareXY = [x, y];
      } else if (color === ISP_SQUARE_COLOR) {
        ispSquareXY = [x, y];
      }

      drawLocationSquare(ctx, color);
      ctx.restore();
    } else if (status === "error" && typeof resp !== "undefined") {
      console.error(resp);
    }
  } else {
    console.error("Couldn't get canvas context");
  }
}

async function sendISPLocation() {
  const response = await fetch(window.location.origin + "/location");
  const json = await response.json();
  drawCurrentLocation(json, ISP_SQUARE_COLOR);
}

function sendExactLocation() {
  if ("geolocation" in navigator) {
    navigator.geolocation.getCurrentPosition(async function (position) {
      let loc = {
        "lat": position.coords.latitude.toString(),
        "lon": position.coords.longitude.toString(),
      }
      const response = await fetch(window.location.origin + "/location", {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify(loc)
      });
      const json = await response.json();
      drawCurrentLocation(json, USER_SQUARE_COLOR);
    }, function(error) {
      console.error("Error getting geolocation:", error);
    });
  } else {
    console.error("Geolocation is not supported by this browser.");
  }
}

async function refreshMap() {
  await drawMap();
  setTimeout(() => {
    if (userSquareXY !== null || ispSquareXY !== null) {
      let canvas = document.getElementById("canvas");
      if (canvas.getContext) {
        const ctx = canvas.getContext("2d");
        if (userSquareXY !== null) {
          ctx.save();
          ctx.translate(userSquareXY[0], userSquareXY[1]);
          ctx.rotate(ANGLE);
          drawLocationSquare(ctx, USER_SQUARE_COLOR);
          ctx.restore();
        }
        if (ispSquareXY !== null) {
          ctx.save();
          ctx.translate(ispSquareXY[0], ispSquareXY[1]);
          ctx.rotate(ANGLE);
          drawLocationSquare(ctx, ISP_SQUARE_COLOR);
          ctx.restore();
        }
      } else {
        console.error("Couldn't get canvas context");
      }
    }
  }, 1000);
}

window.onload = async() => {
  await drawMap();
};