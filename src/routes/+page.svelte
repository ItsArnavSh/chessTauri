<script>
    import { invoke } from '@tauri-apps/api/tauri'
    async function start() {
    let temp = await invoke('start')
  }
</script>
    <a href = "/board" on:click = {start}>Play</a>

