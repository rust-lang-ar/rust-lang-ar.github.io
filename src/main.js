import './styles/tailwind.css';
import './styles/components.css';

import('../pkg').then((mod) => {
  mod.run_app();
});
