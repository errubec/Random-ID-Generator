const seed = Math.trunc(Date.now() / 64 - 1).toString() // does not end in 0
    .split('').reverse().join(''); // does not start with 0
const generatedID = Number(seed).toString(36).toUpperCase();

console.log(generatedID);