<script lang="ts">
import { ref } from 'vue';
import { createActor, dzien1_boot2_backend } from '../../declarations/dzien1_boot2_backend';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent } from '@dfinity/agent';
import type { Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { canisterId} from '../../declarations/dzien1_boot2_backend';
import type { UserData } from '../../declarations/dzien1_boot2_backend/dzien1_boot2_backend.did';

export default {
  data() {
    return {
      newChat: "",
      chats: [] as string[][],
      identity: undefined as undefined | Identity,
      principal: undefined as undefined | Principal,
      targetPrincipal: "",
      userData: undefined as undefined | UserData,
      newUsername: "",
      allUsers: [] as [Principal, UserData][]
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
          const principal = identity.getPrincipal()
          
          this.principal = principal
          this.identity = identity
          
          console.log("Zalogowano", this.principal)
          await this.getUserData()
          await this.getAllUsers()
        }
      })
    },

    async logout() {
      const authClient = await AuthClient.create()
      await authClient.logout()
      
      this.identity = undefined
      this.principal = undefined
      this.chats = []
      this.userData = undefined
    },

    async registerUsername() {
      const trimedUsername = this.newUsername.trim()
      const backend = this.getAuthClient()
      await backend.register(trimedUsername)
      await this.getUserData()
      await this.getAllUsers()
    },

    async getUserData() {
      const {principal} = this.isUserLogged()
      const maybeUserData = await dzien1_boot2_backend.get_user(principal as Principal)
          if (maybeUserData.length === 0 ) {
            this.userData = undefined
          } else {
            this.userData = maybeUserData[0]
          }
          console.log("user data", this.userData)
    },

    async getAllUsers() {
      this.allUsers = await dzien1_boot2_backend.get_users()
    }
  }
}
</script>

<template>
  <main> 
    <button v-if="!principal" @click="login">Login</button>
    <button v-if="principal" @click="logout">Logout</button>
    <div v-if="principal && !userData">
      <input v-model="newUsername" placeholde="Podaj nick"/><button @click="registerUsername">Zapisz Nick</button>
    </div>
    <div v-if="principal && userData">
      {{ userData.nickname }}
      <div v-if="allUsers">
        <select v-model="targetPrincipal">
          <option disabled value="">Please select one</option>
          <option v-for="[userPrincipal, userData] in allUsers" :value="userPrincipal.toText()">{{ userData.nickname }}</option>
        </select>
      </div>
      <div>
        <div v-for="chat in chats[0]">
          {{ chat }}
        </div>
      </div>
      <div>
        <textarea v-model="newChat" placeholder="Wiadomość"></textarea><button @click="dodajChatMSG">Napisz wiadomość</button>
      </div>
    </div>
  </main>
</template>