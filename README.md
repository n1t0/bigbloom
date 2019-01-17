# bigbloom

Memory effective Bloom-filter implementation for huge datasets with Node.js.

This project implements bindings to the [`bloom-rs` rust project](https://github.com/nicklan/bloom-rs) from [nicklan](https://github.com/nicklan/) using the [Neon bindings](https://neon-bindings.com/)

## Motivation

Multiple great projects already implement some Bloom filter for Node.js but they all do it in pure javascript. This works well as long as the filter isn't really big (think (tens/hundreds/++) millions of expected entries).

They work great until you get the famous `JavaScript heap out of memory`.

## Installation

```
npm install bigbloom
```

## Usage

```javascript
const BloomFilter = require('bigbloom');

const capacity  = 1000000;
const errorRate = 0.01;

const filter = new BloomFilter(capacity, errorRate);

filter.contains("foo"); // false
filter.insert("foo");   // true

filter.contains("foo"); // true
filter.insert("foo");   // false, already inserted
```

## TODO
- Add some simple tests
- Add travis for tests and publishing binaries
- Make sure typescript declaration works well
- Update package.json for npm publishing