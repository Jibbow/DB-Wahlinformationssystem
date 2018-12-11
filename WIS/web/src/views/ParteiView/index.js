import React, { Component } from 'react';
import { DropdownButton, MenuItem } from 'react-bootstrap';
import bayern_map from '../../assets/Bayern_Landtagswahlkreise_2018.svg';

export default class ParteiView extends Component {
  constructor(props) {
    super(props);
    this.state = {
      parteien: [],
      selectedParteiId: 0,
    };
  }

  render() {
    return (
      <div class="row">
        <div class="col-xs-6">
          <img src={bayern_map} className="bayern-map" alt="Karte von Bayern" />
        </div>
        <div class="col-xs-6">
          <DropdownButton title={'Wähle eine Partei'} id={'dropdown-parteien'} onSelect={(key, event) => this.setState({ selectedParteiId: key })}>
            {this.state.parteien.map(p => (
              <MenuItem eventKey={p.ID}>{p.NAME}</MenuItem>
            ))}
          </DropdownButton>

          <h2>Knappste Gewinner</h2>

          <h2>Knappste Verlierer</h2>
        </div>
      </div>
    );
  }

  componentDidMount() {
    let start = performance.now();
    fetch('http://localhost:8000/parteien')
      .then(response => response.json())
      .then(data => {
        this.state.parteien = data;
        this.forceUpdate();
      });
  }
}
