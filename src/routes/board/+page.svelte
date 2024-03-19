<script>
// @ts-nocheck

import { invoke } from '@tauri-apps/api/tauri'

//We will make a chessBoard and whenever you click on the board, we send its coordinate to the backend
//Its Coordinates will be in a1-h8 order
let rows = [1,2,3,4,5,6,7,8];
let columns = ['a','b','c','d','e','f','g','h'];
	/**
	 * @type {any}
	 */
let disp
let id = "";
// @ts-ignore
let coord;
//We will have two different Arrays
//First one containing info about the storage of all the pieces
let pieces;

(async () => {
  let object = await invoke('start_game');
  pieces = JSON.parse(object).pieces
  console.log("Pieces1 : ", pieces);
})();

console.log("Pieces2 : ", pieces);
//The Second one Containing possible moves,
let moves = Array.from({length: 8}, () => Array(8).fill(null));//Example, a person clicks on his piece, we have to show the available moves
//Those moves will be shown here
// @ts-ignore
function handleClick(event)
{
    id = event.target.id;
    greet();
}

// @ts-ignore
async function greet() {
    disp = await invoke('greet', { id })
    disp =JSON.parse(disp);
}
</script>

<div class = "mt-20 grid grid-rows-8 grid-cols-8  w-[40%] border-black border-4">
{#if pieces != undefined}
{#each columns as col}
    {#each rows as row}
        <button id = "{col+row}" class = "aspect-[1/1] {(col.charCodeAt(0)-97+row)%2==0?"bg-amber-50 hover:bg-amber-100":"bg-lime-600 hover:bg-lime-500"}  border border-black" on:click = {handleClick}>
            {#if pieces[col.charCodeAt(0)-97][8-row]!="null" && moves[col.charCodeAt(0)-97][row]!=null}
                <img src = {`/static/red.png`} alt = "{pieces[col.charCodeAt(0)-97][row]}">
            {:else if pieces[col.charCodeAt(0)-97][8-row]!="null"}
                <img src = {`${pieces[col.charCodeAt(0)-97][8-row]}`} alt = "{pieces[col.charCodeAt(0)-97][row]}" class = "pointer-events-none {(col.charCodeAt(0)-97+row)%2==0?"bg-amber-50 hover:bg-amber-100":"bg-lime-600 hover:bg-lime-500"}">
            {:else if moves[col.charCodeAt(0)-97][8-row]!=null}
                <img src = {`/static/black.png`} alt = "{pieces[col.charCodeAt(0)-97][row]}">
            {/if}
        </button>
    {/each}
{/each}
{/if}
</div>
<p>{id} was clicked</p>

<p>{disp}</p>
