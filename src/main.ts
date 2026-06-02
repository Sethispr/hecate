import { mount } from 'svelte';
import './styles/_reset.sass';
import './styles/_base.sass';
import './styles/_layout.sass';
import App from './App.svelte';

const root = document.getElementById('root');
if (root) {
  mount(App, { target: root });
}
