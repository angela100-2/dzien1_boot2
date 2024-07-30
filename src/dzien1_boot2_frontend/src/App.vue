<script lang="ts">
import { ref } from 'vue';
import { createActor, dzien1_boot2_backend } from '../../declarations/dzien1_boot2_backend';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent } from '@dfinity/agent';
import type { Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { canisterId} from '../../declarations/dzien1_boot2_backend';

export default {
  data() {
    return {
      newNote: "",
      notes: [] as string[][],
      identity: undefined as undefined | Identity,
      principalText: ""
    }
  },
  methods: {
    async dodajNotatke() {
      if (!this.identity || this.identity.getPrincipal() === Principal.anonymous()) {
        throw new Error("User not log")
      }
      const backend = createActor(canisterId, {
        agentOptions: {
          identity: this.identity
        }
      })
      await backend.add_note(this.newNote)
      await this.pobierzNotatki()
    },

    async pobierzNotatki() {
      if (!this.identity || this.identity.getPrincipal() === Principal.anonymous()) {
        throw new Error("User not log")
      }
      this.notes = await dzien1_boot2_backend.get_notes(this.identity.getPrincipal())
    },

    async login() {
      const authClient = await AuthClient.create()
      await authClient.login({
        identityProvider: "http://avqkn-guaaa-aaaaa-qaaea-cai.localhost:4943/"
      })
      
      const identity = authClient.getIdentity()
      this.principalText = identity.getPrincipal().toText()
      console.log("Zalogowano", identity.getPrincipal())
      this.identity = identity
      await this.pobierzNotatki()
    }
  }
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    {{ principalText }} <button @click="login">Login</button>
      <div>
        <div v-for="note in notes[0]">
          {{ note }}
        </div>
    </div>
    <div>
      <textarea v-model="newNote"></textarea><button @click="dodajNotatke">Dodaj notatke</button>
    </div>
  </main>
</template>