function calculateAngle(hours, minutes) {
    // hours --> 30 degrees
    // minutes --> 6 degrees
    let angHour = 30 * hours;
    let angMinute = 6 * minutes;

    // diff between the two angles
    let angle = Math.abs(angHour - angMinute);

    // angle between two hands --> (360 - angle) and angle
    return Math.min(360 - angle, angle);
}
// Call --> node ./sasha_clock.js
console.log("The angle at 12:00 is " + calculateAngle(12, 0) + " degrees");
console.log("The angle at 3:00 is " + calculateAngle(3, 0) + " degrees");
console.log("The angle at 6:00 is " + calculateAngle(6, 0) + " degrees");
console.log("The angle at 9:00 is " + calculateAngle(9, 0) + " degrees");
console.log("The angle at 12:30 is " + calculateAngle(12, 30) + " degrees");
console.log("The angle at 6:30 is " + calculateAngle(6, 30) + " degrees");
