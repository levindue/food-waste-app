import { Popup } from 'react-leaflet';

function PopupContent({ name, food }) {
  return (
    <Popup>
      <a href='/'>{name}</a>
      <p>Essen: <br /></p>
      <ul>
        {food.map((item, index) => (
          <li key={index}>{item}</li>
        ))}
      </ul>
    </Popup>
  );
}

export default PopupContent;

