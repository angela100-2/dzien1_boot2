<script lang="ts">
import { ref } from 'vue';
import { dzien1_boot2_backend } from '../../declarations/dzien1_boot2_backend';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent } from '@dfinity/agent';
import type { Identity } from '@dfinity/agent';

export default {
  data() {
    return {
      newNote: "",
      notes: [] as string[],
      identity: undefined as undefined | Identity
    }
  },
  methods: {
    async dodajNotatke() {
      await dzien1_boot2_backend.add_note(this.newNote)
      await this.pobierzNotatki()
    },
    async pobierzNotatki() {
      this.notes = await dzien1_boot2_backend.get_notes(Principal.anonymous)
    },
    async login() {
      const authClient = await AuthClient.create()
      await authClient.login({
        identityProvider: "http://avqkn-guaaa-aaaaa-qaaea-cai.localhost:4943/"
      })
      const identity = authClient.getIdentity()
      console.log("Zalogowano", identity.getPrincipal())
      this.identity = identity
      //const agent = new HttpAgent({ identity })
    }
  },
  mounted(){
    this.pobierzNotatki()
  }
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    {{ identity?.getPrincipal() }} <button @click="login">Login</button>
      <div>
      {{ notes }}
    </div>
    <div>
      <textarea v-model="newNote"></textarea><button @click="dodajNotatke">Dodaj notatke</button>
    </div>
  </main>
</template>
