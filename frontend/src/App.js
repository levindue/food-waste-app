import { Routes, Route } from 'react-router-dom'
import Home from './pages/Home';
import './styles/App.css';
import Test from './pages/Test';

function App() {
  return (
    <div className="App">
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/test" element={<Test />} />
      </Routes>
    </div>
  );
}

export default App;
