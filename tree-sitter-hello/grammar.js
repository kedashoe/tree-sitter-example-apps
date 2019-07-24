module.exports = grammar({
  name: 'hello',

  rules: {
    hello: $ => seq(
      $.salutation,
      optional($.ident)
    ),

    salutation: $ => choice('hi', 'bye'),

    ident: $ => {
      const alpha = /[a-zA-Z]/
      return token(seq(alpha, repeat(alpha)))
    },
  }
});

