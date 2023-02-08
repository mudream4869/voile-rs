<template>
  <v-app tabindex="0" @keyup.arrow-left="previous_content" @keyup.arrow-right="next_content">
    <v-breadcrumbs divider="-" :items="breadcrumbsItems">
    </v-breadcrumbs>
    <div v-if="is_text" class="ma-md-2" >
      <h1> {{ content_title }} </h1>

      <span v-for="(line, idx) of content.split('\n')" :key="idx" >
        {{ line }}<br/>
      </span>
    </div>
    <div v-if="!is_text" class="mx-auto">
      <img :src="content_src_url" />
      <!-- prefetch next image -->
      <img :src="next_content_src_url" hidden/>
    </div>
  </v-app>
</template>

<script>
  import { useRoute } from 'vue-router'

  export default {
    data: () => {
      return {
        book: {
          title: '',
          content_titles: [],
          tags: null,
          author: null,
        },

        content_idx: 0,
        book_id: '',
        content: '',
      }
    },
    computed: {
      content_title() {
        return this.book.content_titles[this.content_idx]
      },
      is_text() {
        if (this.book.content_titles.length > this.content_idx) {
          return this.book.content_titles && this.book.content_titles[this.content_idx].endsWith('.txt')
        }
        return false
      },
      content_src_url() {
        return `/api/books/${this.book_id}/contents/${this.content_idx}`
      },
      next_content_src_url() {
        return `/api/books/${this.book_id}/contents/${this.content_idx + 1}`
      },
      breadcrumbsItems() {
        return [
          {
            title: '書櫃',
            link: true,
            disabled: false,
            to: {
              name: 'books',
            },
          },
          {
            title: this.book.title,
            link: true,
            disabled: false,
            to: {
              name: 'book',
              params: {
                book_id: this.book_id,
              },
            },
          },
          {
            title: this.content_title,
            disabled: false,
          },
        ]
      },
    },
    created() {
      const route = useRoute()
      this.content_idx = parseInt(route.params.content_idx)
      this.book_id = route.params.book_id

      // fetch on init
      this.fetchBook()
      this.setBookProc()
    },
    updated() {
      const new_content_idx = parseInt(this.$route.params.content_idx);
      if (new_content_idx != this.content_idx) {
        this.content_idx = new_content_idx
        this.UpdateContentIDX()
      }
    },
    methods: {
      async setBookProc() {
        await fetch(`/api/user/book_proc/${this.book_id}`, {
          method: 'POST',
          body: `${this.content_idx}`,
        })
      },
      async fetchBook() {
        this.book = (await (await fetch(`/api/books/${this.book_id}`)).json())
        if (this.is_text) {
          this.content = (await (await fetch(this.content_src_url)).text())
        }
      },
      async UpdateContentIDX() {
        if (this.is_text) {
          this.content = (await (await fetch(this.content_src_url)).text())
        }

        this.$router.push({
          name: 'content',
          params: {
            book_id: this.book_id,
            content_idx: this.content_idx,
          }
        })

        this.setBookProc()
      },
      previous_content() {
        if (this.content_idx > 0) {
          this.content_idx -= 1
          this.UpdateContentIDX()
        }
      },
      next_content() {
        if (this.content_idx + 1 < this.book.content_titles.length) {
          this.content_idx += 1
          this.UpdateContentIDX()
        }
      },
    },
  }
</script>
