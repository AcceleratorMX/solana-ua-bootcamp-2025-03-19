const hexInput = (input) => input;

const decodedString = hexInput("4920616D206E6F7420612068616D73746572")
    .replace(/\s/g, '')
    .match(/.{1,2}/g)
    .map(hexPair => String.fromCharCode(parseInt(hexPair, 16)))
    .join('');

console.log('Decoded String:', decodedString);