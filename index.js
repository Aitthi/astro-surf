/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

const astroSurf = require("./astro-surf")

const AstroSurf = {
  initialize: (handler, opt = {
    client_path: './dist/client',
  }) => {
    let surf = astroSurf.AppFactory.initialize(async (ctx) => {
      let body = []
      let status = 200;
      let headers = {};
      let astrohandler = () => {
        return new Promise((resolve) => {
          ctx.response['writeHead'] = (status, header) => {
            status = status;
            headers = { ...headers, ...header }
          }
          ctx.response['write'] = (uint8_array) => {
            body.push(...uint8_array);
          }
          ctx.response['end'] = () => {
            resolve(true);
          }
          handler(ctx.request, ctx.response);
        })
      }
      await astrohandler();
      return [
        status,
        headers,
        body
      ];
    }, opt.client_path || "./dist/client");
    return {
      serve: async (port, host = '127.0.0.1') => {
        await surf.serve(port, host)
      }
    }
  }
}

module.exports = AstroSurf