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
      newChat: "",
      chats: [] as string[][],
      identity: undefined as undefined | Identity,
      principalText: "",
      targetPrincipal: ""
    }
  },
  methods: {
    async dodajChatMSG() {
      if (!this.identity || this.identity.getPrincipal() === Principal.anonymous()) {
        throw new Error("User not log")
      }

      const targetPrincipal = Principal.fromText(this.targetPrincipal)
      if (!targetPrincipal || targetPrincipal === Principal.anonymous()) {
        throw new Error("Who u writing to")
      }

      const backend = createActor(canisterId, {
        agentOptions: {
          identity: this.identity
        }
      })

      await backend.add_chat_msg(this.newChat, targetPrincipal)
      await this.pobierzChaty()
    },

    async pobierzChaty() {
      if (!this.identity || this.identity.getPrincipal() === Principal.anonymous()) {
        throw new Error("User not log")
      }
      
      const targetPrincipal = Principal.fromText(this.targetPrincipal)
      if (!targetPrincipal || targetPrincipal === Principal.anonymous()) {
        throw new Error("Who u writing to")
      }
      
      this.chats = await dzien1_boot2_backend.get_chat(this.identity.getPrincipal(), targetPrincipal)
    },

    async login() {
      const authClient = await AuthClient.create()
      await authClient.login({
        identityProvider: "http://avqkn-guaaa-aaaaa-qaaea-cai.localhost:4943/"
      })
      
      const identity = authClient.getIdentity()
      this.principalText = identity.getPrincipal().toText()
      console.log("Zalogowano", this.principalText)
      this.identity = identity
      await this.pobierzChaty()
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
      <input v-model="targetPrincipal" />
    </div>
      <div>
        <div v-for="chat in chats[0]">
          {{ chat }}
        </div>
    </div>
    <div>
      <textarea v-model="newChat"></textarea><button @click="dodajChatMSG">Napisz wiadomość</button>
    </div>
  </main>
</template>