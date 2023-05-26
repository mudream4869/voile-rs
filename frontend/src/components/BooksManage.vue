<template>
  <v-app>
    <v-container fluid>
      <v-row>
        <v-col>
          <v-chip class="ma-2" v-for="btype in book_types" :key="btype" :color="book_type == btype ? 'red' : 'default'"
            @click="toggleBookType(btype)" label> {{ btype }} </v-chip>
        </v-col>

        <v-col>
          <v-chip class="ma-2" v-for="tag in tags" :key="tag" :color="used_tags.has(tag) ? 'green' : 'default'"
            @click="toggleTag(tag)" label> {{ tag }} </v-chip>
        </v-col>
      </v-row>
      <v-table>
        <thead>
          <tr>
            <th class="text-left">
              Select
            </th>
            <th class="text-left">
              Book Type
            </th>
            <th class="text-left">
              Title
            </th>
            <th class="text-left">
              Author
            </th>
            <th class="text-left">
              Tags
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="book in show_books" :key="book.book_id">
            <td class="text-xs-center">
              <v-checkbox class="d-inline-flex" :value="book.book_id" v-model="selected_bookids">
              </v-checkbox>
            </td>
            <td>{{ book.book_type }}</td>
            <td>{{ book.title }}</td>
            <td>{{ book.author }}</td>
            <td>
              <v-chip class="ma-2" label v-for="tag in book.tags" :key="tag"> {{ tag }} </v-chip>
            </td>
          </tr>
        </tbody>
      </v-table>
    </v-container>
  </v-app>
</template>

<script>
export default {
  data: () => {
    return {
      selected_bookids: [],
      books: [],
      book_types: [],
      book_type: null,

      tags: [],
      used_tags: new Set(),
    }
  },
  created() {
    // fetch on init
    this.fetchData()
  },
  computed: {
    show_books() {
      return this.books.filter(book => {
        var match = true
        this.used_tags.forEach(tag => {
          if (!book.tags_set.has(tag)) {
            match = false
          }
        })

        if (this.book_type == null) {
          // ok
        } else if (this.book_type == '<NULL>' && book.book_type == undefined) {
          // ok
        } else if (this.book_type == book.book_type) {
          // ok
        } else {
          match = false
        }

        return match
      })
    }
  },
  methods: {
    async fetchData() {
      this.books = (await (await fetch('/api/books')).json()).books.map(book => {
        book.tags_set = new Set(book.tags || [])
        return book
      })
      this.tags = [...new Set(this.books.map(book => book.tags || []).flat())]
      this.book_types = [...new Set(this.books.map(book => book.book_type || '<NULL>'))]
    },
    toggleTag(tag) {
      if (this.used_tags.has(tag)) {
        this.used_tags.delete(tag)
      } else {
        this.used_tags.add(tag)
      }
    },
    toggleBookType(btype) {
      if (this.book_type == btype) {
        this.book_type = null
      } else {
        this.book_type = btype
      }
    }
  },
}
</script>
