import createCache from '@emotion/cache';
//https://medium.com/nextjs/how-to-use-material-ui-with-nextjs-and-react-18-6c054ceacf77
export default function createEmotionCache() {
    return createCache({ key: 'css', prepend: true });
}