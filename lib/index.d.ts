declare module "bigbloom" {
	export class BloomFilter {
		capacity: number;
		errorRate: number;
		bits: number;
		hashes: number;
		
		constructor(capacity: number, errorRate: number);
		
		insert(element: string | number): boolean;
		contains(element: string | number): boolean;
		clear(): void;
	}
}