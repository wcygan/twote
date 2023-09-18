import FetchDataComponent from './components/HelloComponent';
import LoginComponent from './components/LoginComponent';

function App() {
  return (
      <div>
        <h1>Hello Request:</h1>
        <FetchDataComponent />
        <h1>Login Request:</h1>
        <LoginComponent />
      </div>
  );
}

export default App;
