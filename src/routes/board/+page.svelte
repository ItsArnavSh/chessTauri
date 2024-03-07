<script>
import { invoke } from '@tauri-apps/api/tauri'

//We will make a chessBoard and whenever you click on the board, we send its coordinate to the backend
//Its Coordinates will be in a1-h8 order
let rows = [8,7,6,5,4,3,2,1];
let columns = ['a','b','c','d','e','f','g','h'];
let id = "";
// @ts-ignore
let coord;
//We will have two different Arrays
//First one containing info about the storage of all the pieces
let pieces = Array.from({length: 8}, () => Array(8).fill(null));//A array that contains info of where which image is
//The Second one Containing possible moves,
let moves = Array.from({length: 8}, () => Array(8).fill(null));//Example, a person clicks on his piece, we have to show the available moves
//Those moves will be shown here
// @ts-ignore
function handleClick(event)
{
    id = event.target.id;
    greet(id);
}
// @ts-ignore
async function greet(coord) {
    let temp = await invoke('greet', { coord })
    if(temp.message != "0")
    {
        temp = JSON.parse(temp);
        [pieces,moves] = temp.val;
    }
  }
</script>
<div class = "mt-20 grid grid-rows-8 grid-cols-8  w-[40%] border-black border-4">
{#each rows as row}
    {#each columns as col}
        <button id = "{col+row}" class = "aspect-[1/1] {(col.charCodeAt(0)-97+row)%2==0?"bg-amber-50 hover:bg-amber-100":"bg-lime-600 hover:bg-lime-500"}  border border-black" on:click = {handleClick}>
            {#if pieces[col.charCodeAt(0)-97][row]!=null && moves[col.charCodeAt(0)-97][row]!=null}
                <img src = {`/static/red.png`} alt = "{pieces[col.charCodeAt(0)-97][row]}">
            {:else if pieces[col.charCodeAt(0)-97][row]!=null}
                <img src = {`/static/${pieces[col.charCodeAt(0)-97-97][8-row]}.png`} alt = "{pieces[col.charCodeAt(0)-97][row]}">
            {:else if moves[col.charCodeAt(0)-97][row]!=null}
                <img src = {`/static/black.png`} alt = "{pieces[col.charCodeAt(0)-97][row]}">
            {/if}
        </button>
    {/each}
{/each}
</div>
<p>{id} was clicked</p>