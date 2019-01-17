var addon   = require('../native');

class BloomFilter {
	/**
	 * Construct a BloomFilter
	 * @param {int}   capacity  The filter's capacity is the expected number of elements it will contain.
	 * @param {float} errorRate The error rate represents the "false positive" rate to target. [default = 1%]
	 */
	constructor(capacity, errorRate = 0.01) {
		this._filter = new addon.BloomFilter(capacity, errorRate);
	}
	
	get capacity() {
		return this._filter.expected_size();
	}
	
	get errorRate() {
		return this._filter.false_positive_rate();
	}
	
	get bits() {
		return this._filter.num_bits();
	}
	
	get hashes() {
		return this._filter.num_hashes();
	}
	
	_prepare(element) {
		if (typeof element === "number" && parseInt(element, 10) !== element) {
			return String(element);
		}
		return element;
	}
	
	/**
	 * Add an element to the bloom filter
	 * @param {int | string} element the element to add
	 * @return {boolean} true if the element was inserted, false if already there
	 */
	insert(element) {
		return this._filter.insert(this._prepare(element));
	}
	
	/**
	 * Check whether an elements is already in the filter
	 * @param {int | string} element the element to check
	 * @return {boolean} whether the element is in the filter
	 */
	contains(element) {
		return this._filter.contains(this._prepare(element));
	}
	
	/**
	 * Empty the filter of all its elements
	 */
	clear() {
		this._filter.clear();
	}
}

module.exports = BloomFilter;