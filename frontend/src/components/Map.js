import { MapContainer, TileLayer, Marker, Popup } from 'react-leaflet';
import './Map.css';
import 'leaflet/dist/leaflet.css'
import { Icon } from 'leaflet';

const myIcon = new Icon({
  iconUrl: "https://unpkg.com/leaflet@1.9.3/dist/images/marker-icon.png",
  iconSize: [25, 41]
})

function Map() {
  const position = [51.97509, 7.40953]

  return (
    <div className="Map">
      <MapContainer center={position} zoom={15} scrollWheelZoom={false}>
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />
        <Marker position={position} icon={myIcon}>
          <Popup>
            <a href='/'>Levin Düsterhus</a>
            <p>Essen: <br /></p>
            <ul>
              <li>Brot</li>
              <li>3 Äpfel</li>
              <li>2 Bananen</li>
            </ul>
          </Popup>
        </Marker>
      </MapContainer>
    </div>
  )
}

export default Map;
