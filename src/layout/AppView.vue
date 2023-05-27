<template>
    <div class="app-view">
        <router-view @context-selected="onContextSelected" @view-logs="$emit('view-logs', $event)" />
    </div>
</template>
  
<script>
export default {
    props: {
        onContextSelected: {
            type: Function,
            required: true,
        },
    },
};
</script>
<style>
</style>
  