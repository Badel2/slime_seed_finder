declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	* @param {any} o
	* @returns {string}
	*/
	export function slime_seed_finder(o: any): string;
	/**
	* @param {string} o
	* @returns {any[]}
	*/
	export function river_seed_finder(o: string): any[];
	/**
	* @param {string} o
	* @returns {any}
	*/
	export function draw_rivers(o: string): any;
	/**
	* Returns `DrawRivers` object
	* @param {string} o
	* @returns {any}
	*/
	export function generate_rivers_candidate(o: string): any;
	/**
	* @param {any} o
	* @returns {string}
	*/
	export function count_candidates(o: any): string;
	/**
	* @param {any} o
	* @returns {Uint8Array}
	*/
	export function draw_reverse_voronoi(o: any): Uint8Array;
	/**
	* @param {string} s
	* @returns {string}
	*/
	export function extend48(s: string): string;
	/**
	* @param {string} o
	* @returns {string}
	*/
	export function count_rivers(o: string): string;
	/**
	* @param {string} version
	* @param {number} fx
	* @param {number} fy
	* @param {string} seed
	* @param {number} frag_size
	* @param {number} y_offset
	* @returns {Uint8Array}
	*/
	export function generate_fragment(version: string, fx: number, fy: number, seed: string, frag_size: number, y_offset: number): Uint8Array;
	/**
	* @param {string} version
	* @param {number} fx
	* @param {number} fy
	* @param {string} seed
	* @param {number} frag_size
	* @param {number} layer
	* @param {number} y_offset
	* @returns {Uint8Array}
	*/
	export function generate_fragment_up_to_layer(version: string, fx: number, fy: number, seed: string, frag_size: number, layer: number, y_offset: number): Uint8Array;
	/**
	* @param {number} fx
	* @param {number} fy
	* @param {any[]} seeds
	* @param {number} frag_size
	* @returns {Uint8Array}
	*/
	export function generate_fragment_slime_map(fx: number, fy: number, seeds: any[], frag_size: number): Uint8Array;
	/**
	* @param {string} seed
	* @returns {string}
	*/
	export function is_i64(seed: string): string;
	/**
	* @param {string} seed
	* @param {number} n
	* @returns {string}
	*/
	export function add_2_n(seed: string, n: number): string;
	/**
	* @param {string} seed
	* @param {number} n
	* @returns {string}
	*/
	export function sub_2_n(seed: string, n: number): string;
	/**
	* @param {string} base
	* @param {string} n
	* @param {string} bits
	* @returns {string}
	*/
	export function gen_test_seed_base_n_bits(base: string, n: string, bits: string): string;
	/**
	* @param {string} seed
	* @returns {string}
	*/
	export function similar_biome_seed(seed: string): string;
	/**
	* @param {string} o
	* @returns {Uint8Array}
	*/
	export function draw_treasure_map(o: string): Uint8Array;
	/**
	* @param {string} o
	* @returns {any[]}
	*/
	export function treasure_map_seed_finder(o: string): any[];
	/**
	* @param {File} zip_file
	* @param {boolean} is_minecraft_1_15
	* @returns {string}
	*/
	export function anvil_region_to_river_seed_finder(zip_file: File, is_minecraft_1_15: boolean): string;
	/**
	* Returns `Option<ExtractMapResult>`
	* @param {number} width
	* @param {number} height
	* @param {Uint8Array} screenshot
	* @returns {any}
	*/
	export function extract_map_from_screenshot(width: number, height: number, screenshot: Uint8Array): any;
	/**
	* Returns `Vec<FoundDungeon>`
	* @param {File} zip_file
	* @returns {any[]}
	*/
	export function read_dungeons(zip_file: File): any[];
	/**
	* Returns `Vec<Position>`
	* @param {File} zip_file
	* @param {any} params
	* @returns {any[]}
	*/
	export function find_blocks_in_world(zip_file: File, params: any): any[];
	/**
	* Returns `Vec<Position>`
	* @param {File} zip_file
	* @param {any} params
	* @returns {any[]}
	*/
	export function find_block_pattern_in_world(zip_file: File, params: any): any[];
	/**
	* Returns `Vec<FindMultiDungeonsOutput>`
	* @param {File} zip_file
	* @param {any} params
	* @returns {any[]}
	*/
	export function find_spawners_in_world(zip_file: File, params: any): any[];
	/**
	* Returns `Vec<NbtSearchResult>`
	* @param {File} _zip_file
	* @param {string} _block_name
	* @returns {any[]}
	*/
	export function nbt_search(_zip_file: File, _block_name: string): any[];
	/**
	* Returns HashMap<String, i32>
	* @returns {any}
	*/
	export function get_color_to_biome_map(): any;
	/**
	* Returns HashMap<String, String>
	* @returns {any}
	*/
	export function get_biome_id_to_biome_name_map(): any;
	/**
	* @param {File} zip_file
	* @param {string} version_str
	* @param {number} fx
	* @param {number} fy
	* @param {number} frag_size
	* @param {number} y_offset
	* @returns {Uint8Array}
	*/
	export function read_fragment_biome_map(zip_file: File, version_str: string, fx: number, fy: number, frag_size: number, y_offset: number): Uint8Array;
	/**
	* @param {Map<any, any>} fragments
	* @param {number} tsize
	* @returns {string}
	*/
	export function map_fragments_to_png_base64(fragments: Map<any, any>, tsize: number): string;
	/**
	*/
	export function init(): void;
	/**
	*/
	export class DrawRivers {
	  free(): void;
	}
	/**
	*/
	export class GenerateRiversCandidate {
	  free(): void;
	}
	/**
	*/
	export class Options {
	  free(): void;
	}
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_options_free: (a: number) => void;
  readonly __wbg_drawrivers_free: (a: number) => void;
  readonly __wbg_generateriverscandidate_free: (a: number) => void;
  readonly slime_seed_finder: (a: number, b: number) => void;
  readonly river_seed_finder: (a: number, b: number, c: number) => void;
  readonly draw_rivers: (a: number, b: number) => number;
  readonly generate_rivers_candidate: (a: number, b: number) => number;
  readonly count_candidates: (a: number, b: number) => void;
  readonly draw_reverse_voronoi: (a: number, b: number) => void;
  readonly extend48: (a: number, b: number, c: number) => void;
  readonly count_rivers: (a: number, b: number, c: number) => void;
  readonly generate_fragment: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly generate_fragment_up_to_layer: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => void;
  readonly generate_fragment_slime_map: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly is_i64: (a: number, b: number, c: number) => void;
  readonly add_2_n: (a: number, b: number, c: number, d: number) => void;
  readonly sub_2_n: (a: number, b: number, c: number, d: number) => void;
  readonly gen_test_seed_base_n_bits: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly similar_biome_seed: (a: number, b: number, c: number) => void;
  readonly draw_treasure_map: (a: number, b: number, c: number) => void;
  readonly treasure_map_seed_finder: (a: number, b: number, c: number) => void;
  readonly anvil_region_to_river_seed_finder: (a: number, b: number, c: number) => void;
  readonly extract_map_from_screenshot: (a: number, b: number, c: number, d: number) => number;
  readonly read_dungeons: (a: number, b: number) => void;
  readonly find_blocks_in_world: (a: number, b: number, c: number) => void;
  readonly find_block_pattern_in_world: (a: number, b: number, c: number) => void;
  readonly find_spawners_in_world: (a: number, b: number, c: number) => void;
  readonly nbt_search: (a: number, b: number, c: number, d: number) => void;
  readonly get_color_to_biome_map: () => number;
  readonly get_biome_id_to_biome_name_map: () => number;
  readonly read_fragment_biome_map: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly map_fragments_to_png_base64: (a: number, b: number, c: number) => void;
  readonly init: () => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
