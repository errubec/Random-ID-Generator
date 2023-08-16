const timestamp /* Number */ = Math.floor(Date.now() / 1000); // Number of seconds since 0:00 UTC on 1 Jan 1970
const randomDigit /* Number */ = Math.floor(Math.random() * 9) + 1; // Random digit between 1 and 9, inclusive
const seed /* String */ = `${timestamp}${randomDigit}`.split('').reverse().join('');
const generatedID /* String */ = Number(seed).toString(36).toUpperCase();

console.log(generatedID);