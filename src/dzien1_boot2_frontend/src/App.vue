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
      principal: undefined as undefined | Principal,
      targetPrincipal: ""
    }
  },
  methods: {
    isUserLogged() {
      if (!this.identity || !this.principal ||this.principal === Principal.anonymous()) {
        throw new Error("User not log")
      }
      return {
        identity: this.identity,
        principal: this.principal
      }
    },

    validateTargetPrincipal() {
      const cleanTargetPrincipal = this.targetPrincipal.trim()
      if(cleanTargetPrincipal === "") {
        throw new Error ("No principal?")
      }
      const targetPrincipal = Principal.fromText(cleanTargetPrincipal)
      if (!targetPrincipal || targetPrincipal === Principal.anonymous()) {
        throw new Error("Who u writing to")
      }
      return targetPrincipal
    },

    getAuthClient() {
      this.isUserLogged()
      return createActor(canisterId, {
        agentOptions: {
          identity: this.identity
        }
      })
    },

    async dodajChatMSG() {
      const targetPrincipal = this.validateTargetPrincipal()
      const backend = this.getAuthClient()

      await backend.add_chat_msg(this.newChat, targetPrincipal)
      await this.pobierzChaty()
    },

    async pobierzChaty() {
      const {identity, principal} = this.isUserLogged()
      const targetPrincipal = this.validateTargetPrincipal()
      
      const chatPath = [targetPrincipal, identity.getPrincipal()].sort()
      this.chats = await dzien1_boot2_backend.get_chat(chatPath)
    },

    async login() {
      const authClient = await AuthClient.create()
      await authClient.login({
        identityProvider: "http://avqkn-guaaa-aaaaa-qaaea-cai.localhost:4943/",
        onSuccess: async () => {
          const identity = authClient.getIdentity()
          this.principal = identity.getPrincipal()
          console.log("Zalogowano", this.principal)
          this.identity = identity
          await this.pobierzChaty()
        }
      })
    }
  }
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    {{ principal }} <button @click="login">Login</button>
    <div>
      <input v-model="targetPrincipal" /><button @click="pobierzChaty">Pobierz chat</button>
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