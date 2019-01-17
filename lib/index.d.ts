declare class BloomFilter {
	constructor(capacity: number, errorRate: number = 0.01);
	
	get capacity(): number;
	get errorRate(): number;
	get bits(): number;
	get hashes(): number;
	
	insert(element: string | number): boolean;
	contains(element: string | number): boolean;
	clear();
}