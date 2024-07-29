<script lang="ts">
import { ref } from 'vue';
import { dzien1_boot2_backend } from '../../declarations/dzien1_boot2_backend';

export default {
  data() {
    return {
      newNote: "",
      notes: [] as string[]
    }
  },
  methods: {
    async dodajNotatke() {
      dzien1_boot2_backend.add_note(this.newNote)
      await this.pobierzNotatki()
    },
    async pobierzNotatki() {
      this.notes = await dzien1_boot2_backend.get_notes()
    }
  },
  mounted() {
    this.pobierzNotatki()
  }
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <div>
      {{ notes }}
    </div>
    <div>
      <textarea v-model="newNote"><button @click="dodajNotatke">Dodaj notatkę</button></textarea>
    </div>
  </main>
</template>
