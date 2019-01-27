import React, { Component } from 'react';
import { Button, Modal, FormGroup, ControlLabel, FormControl, Radio, Table } from 'react-bootstrap';

export class VoteButton extends Component {
  constructor(props, context) {
    super(props, context);

    this.handleIdentity = this.handleIdentity.bind(this);
    this.handleErststimme = this.handleErststimme.bind(this);
    this.handleErststimmeAbgeben = this.handleErststimmeAbgeben.bind(this);
    this.handleErststimmeEnthalten = this.handleErststimmeEnthalten.bind(this);
    this.handleZweitstimme = this.handleZweitstimme.bind(this);
    this.handleZweitstimmeAbgeben = this.handleZweitstimmeAbgeben.bind(this);
    this.handleZweitstimmeEnthalten = this.handleZweitstimmeEnthalten.bind(this);
    this.handleAbschluss = this.handleAbschluss.bind(this);
    this.handleClose = this.handleClose.bind(this);
    this.vote = this.vote.bind(this);
    this.validateWahltokenState = this.validateWahltokenState.bind(this);
    this.onErststimmeAuswahl = this.onErststimmeAuswahl.bind(this);
    this.onZweitstimmeKandidatAuswahl = this.onZweitstimmeKandidatAuswahl.bind(this);
    this.onZweitstimmeParteiAuswahl = this.onZweitstimmeParteiAuswahl.bind(this);

    this.state = {
      identity: false,
      erststimme: false,
      zweitstimme: false,
      abschluss: false,
      wahltoken: '',
      zweitstimmekandidaten: [], 
      erststimmekandidaten: [],
      jahr: 0,
      stimmkreis: 0,
      erststimmeabgegeben: 0,
      zweitstimmeabgegeben: 0,
      erststimmeenthaltung: false,
      erststimmewahl: -1,
      zweitstimmeenthaltung: false,
      zweitstimmewahlpartei: -1,
      zweitstimmewahlkandidat: -1
    };
  }

  vote() {
    console.log("Enthaltung? " + this.state.zweitstimmeenthaltung);
    console.log("Auswahl? " + this.state.zweitstimmewahlkandidat);
    let erststimme = '';
    let zweitstimme = '';
    if (this.state.erststimmeenthaltung === true) {
      erststimme = 'enthaltung';
    }
    else if (this.state.erststimmewahl === -1) {
      erststimme = null;
    }
    else {
      erststimme = {
        'kandidat': this.state.erststimmewahl
      };
    }
    if (this.state.zweitstimmeenthaltung === true) {
      zweitstimme = 'enthaltung';
    }
    else if (this.state.zweitstimmewahlkandidat > -1) {
      zweitstimme = {
        'kandidat': this.state.zweitstimmewahlkandidat
    }
    }
    else if (this.state.zweitstimmewahlpartei > -1) {
      zweitstimme = {
        'partei': this.state.zweitstimmewahlpartei
    }
    }
    else {
      zweitstimme = null
    }
    let ergebnis = {
      'token': this.state.wahltoken,
      'erststimme': erststimme,
      'zweitstimme': zweitstimme
    };
    console.log(ergebnis);
    return fetch('http://localhost:8000/abstimmen', {
      method: 'POST',
      body: JSON.stringify(ergebnis)
    })
    .then(function (data) {  
      console.log('Request success: ', data);  
    })  
    .catch(function (error) {  
      console.log('Request failure: ', error);  
    });
  }
  

  validateWahltokenState() {
    fetch(`http://localhost:8000/tokeninfo/`.concat(this.state.wahltoken)) 
    .then(response => response.json())
    .then(data => {
      if (data.length == 1) {
          let tokeninfo = data[0];
          this.setState({stimmkreis: tokeninfo.STIMMKREIS});
          this.setState({jahr: tokeninfo.JAHR});
          this.setState({erststimmeabgegeben: tokeninfo.ERSTSTIMMEABGEGEBEN});
          this.setState({zweitstimmeabgegeben: tokeninfo.ZWEITSTIMMEABGEGEBEN});
          fetch(`http://localhost:8000/wahlzettel/erststimme/${this.state.stimmkreis}/${this.state.jahr}`)
          .then(response => response.json())
          .then(data => {
            this.setState({ erststimmekandidaten: data});
          });
          fetch(`http://localhost:8000/wahlzettel/zweitstimme/${this.state.stimmkreis}/${this.state.jahr}`)
          .then(response => response.json())
          .then(data => {
            let parteikandidaten_temp = this.getParteikandidaten(data);
            this.setState({ zweitstimmekandidaten: parteikandidaten_temp});
          });
          if (this.state.erststimmeabgegeben == 0) {
            this.handleErststimme();
          }
          else if (this.state.zweitstimmeabgegeben == 0) {
            this.handleZweitstimme();
          }
          else {
            window.error("Sie haben schon eine Erststimme und eine Zweitstimme abgegeben.");
          }
      }
      else {
        window.alert("Ihr Wahltoken ist ungültig1.");
      }
    })
    .catch(error => {
      window.alert("Ihr Wahltoken ist ungültig2.");
    });

  }

  validateWahltoken() {
    if (this.state.wahltoken.match('^[0-9a-z]{8}-[0-9a-z]{4}-[0-9a-z]{4}-[0-9a-z]{4}-[0-9a-z]{12}$')) {
      return 'success';
    }
    return 'error';
  }

  getParteikandidaten(data) {
    let parteikandidaten_temp = [];
    data.map(p => {
      let gefunden = false;
      for (const pk of parteikandidaten_temp) {
        if (pk.partei.abkuerzung === p.PARTEI_ABKUERZUNG) {
          pk.kandidaten.push({id: p.KANDIDAT_ID, position: p.LISTENPOSITION, name: p.KANDIDAT_NACHNAME, vorname: p.KANDIDAT_VORNAME});
          gefunden = true
        }
      }
      if (!gefunden) {
        parteikandidaten_temp.push({partei: { id: p.PARTEI_ID, name: p.PARTEI, abkuerzung: p.PARTEI_ABKUERZUNG }, kandidaten: [{id: p.KANDIDAT_ID, position: p.LISTENPOSITION, name: p.KANDIDAT_NACHNAME, vorname: p.KANDIDAT_VORNAME}]});
      }
    });
    return parteikandidaten_temp;
  }

  getPositionskandidaten() {
    let maxLength = 0;
    for (const pk of this.state.zweitstimmekandidaten) {
      if (pk.kandidaten.length > maxLength) {
        maxLength = pk.kandidaten.length;
      }
    }

    let positionskandidaten = [];
    for (let k = 0; k < maxLength; k++) {
      positionskandidaten.push([]);
    }

    this.state.zweitstimmekandidaten.map((pk, j) => {
      for (let i = 0; i < maxLength; i++) {
        if (pk.kandidaten.length > i){
          positionskandidaten[i].push(pk.kandidaten[i]);
        }
      }
    });
    return positionskandidaten;
  }

  render() {
    return (
      <div>
        <Button className="vote-button" bsSize="large" bsStyle="primary" onClick={this.handleIdentity}>
          Meine Stimme abgeben
        </Button>

        <Modal show={this.state.identity} onHide={this.handleClose}>
          <Modal.Header closeButton>
            <Modal.Title>Meine Stimme abgeben</Modal.Title>
          </Modal.Header>
          <Modal.Body>
              <FormGroup controlId="formWahltoken" validationState={this.validateWahltoken()}>
                <ControlLabel>Geben Sie bitte hier Ihren Wahltoken ein:</ControlLabel>
                <FormControl type="text" value={this.state.wahltoken} placeholder="z.B. 00000000-0000-0000-0000-000000000000" onChange={e => this.setState({ wahltoken: e.target.value })} />
                <FormControl.Feedback />
              </FormGroup>
          </Modal.Body>
          <Modal.Footer>
            <Button onClick={this.validateWahltokenState}>Wahl durchführen</Button>
          </Modal.Footer>
        </Modal>

        <Modal show={this.state.erststimme} onHide={this.handleClose}>
          <Modal.Header closeButton>
            <Modal.Title>Sie haben eine Stimme</Modal.Title>
          </Modal.Header>
          <Modal.Body>
            <FormGroup>
            <div className = "stimmabgabeTable">
              <Table>
                <thead>
                  <tr>
                    {this.state.erststimmekandidaten.map((k, i) =>
                      <th>
                        <div>Wahlvorschlag Nr. {i + 1}</div>
                        <div>{k.PARTEI}</div>
                        <div>{k.PARTEI_ABKUERZUNG}</div>
                      </th>
                    )}
                  </tr>
                </thead>
                <tbody>
                  <tr>
                  {this.state.erststimmekandidaten.map(k =>
                    <td><Radio name="radioGroup" value={k.KANDIDAT_ID} onChange={this.onErststimmeAuswahl} inline>
                      <div>{k.LISTENPOSITION}</div>
                      <div>{k.KANDIDAT_NACHNAME}</div>
                      <div>{k.KANDIDAT_VORNAME}</div>
                    </Radio></td>
                  )}
                  </tr>
                </tbody>
              </Table>
            </div>
            </FormGroup>
          </Modal.Body>
          <Modal.Footer>
          <Button onClick={this.handleErststimmeEnthalten}>Erststimme enthalten</Button>
            <Button onClick={this.handleErststimmeAbgeben}>Erststimme abgeben</Button>
          </Modal.Footer>
        </Modal>

        <Modal show={this.state.zweitstimme} onHide={this.handleClose}>
          <Modal.Header closeButton>
            <Modal.Title>Sie haben eine Stimme</Modal.Title>
          </Modal.Header>
          <Modal.Body>
            <FormGroup>
              <div className="stimmabgabeTable">
                <Table striped bordered condensed hover>
                  <thead>
                    <tr>
                      {this.state.zweitstimmekandidaten.map((p, i) =>
                        <th><Radio name="radioGroup" value={p.partei.id} onChange={this.onZweitstimmeParteiAuswahl} inline>
                          <div>Wahlvorschlag Nr. {i + 1}</div>
                          <div>{p.partei.id}</div>
                          <div>{p.partei.abkuerzung}</div>
                        </Radio></th>
                      )}
                    </tr>
                  </thead>
                  <tbody>
                    {this.getPositionskandidaten().map((p, i) =>
                      <tr>
                        {p.map((k, j) =>
                          <td><Radio name="radioGroup" value={k.id} onChange={this.onZweitstimmeKandidatAuswahl} inline>
                            <div>{k.position} {k.name}, {k.vorname}</div>
                          </Radio></td>
                        )}
                      </tr>
                    )}
                  </tbody>
                </Table>
              </div>
            </FormGroup>
          </Modal.Body>
          <Modal.Footer>
            <Button onClick={this.handleZweitstimmeEnthalten}>Zweitstimme enthalten</Button>
            <Button onClick={this.handleZweitstimmeAbgeben}>Zweitstimme abgeben</Button>
          </Modal.Footer>
        </Modal>

        <Modal show={this.state.abschluss} onHide={this.handleClose}>
          <Modal.Header closeButton>
            <Modal.Title>
              <div>Vielen Dank.</div>
              <div>Ihre Stimme wurde gespeichert.</div>
            </Modal.Title>
          </Modal.Header>
        </Modal>

      </div>
    );
  }

  onErststimmeAuswahl(e) {
    this.setState({ erststimmewahl: parseInt(e.currentTarget.value) });
  }

  onZweitstimmeKandidatAuswahl(e) {
    this.setState({ zweitstimmewahlkandidat: parseInt(e.currentTarget.value) });
    this.setState({ zweitstimmewahlpartei: -1 });
  }

  onZweitstimmeParteiAuswahl(e) {
    this.setState({ zweitstimmewahlpartei: parseInt(e.currentTarget.value) });
    this.setState({ zweitstimmewahlkandidat: -1 });
  }

  handleClose() {
    this.setState({ identity: false });
    this.setState({ erststimme: false });
    this.setState({ zweitstimme: false });
    this.setState({ abschluss: false });
  }

  handleIdentity() {
    this.setState({ identity: true });
    this.setState({ erststimme: false });
    this.setState({ zweitstimme: false });
    this.setState({ abschluss: false });
  }

  handleErststimme() {
    this.setState({ identity: false });
    this.setState({ erststimme: true });
    this.setState({ zweitstimme: false });
    this.setState({ abschluss: false });
  }

  handleErststimmeAbgeben() {
    this.setState({ erststimmeenthaltung: false });
    this.setState({ erststimmeabgegeben: 1 });
    if (this.state.zweitstimmeabgegeben === 0) {
      this.handleZweitstimme();
    }
    else {
      this.handleAbschluss();
    }
  }

  handleErststimmeEnthalten() {
    this.setState({ erststimmeenthaltung: true });
    this.setState({ erststimmeabgegeben: 1 });
    if (this.state.zweitstimmeabgegeben === 0) {
      this.handleZweitstimme();
    }
    else {
      this.handleAbschluss();
    }
  }

  handleZweitstimme() {
    this.setState({ identity: false });
    this.setState({ erststimme: false });
    this.setState({ zweitstimme: true });
    this.setState({ abschluss: false });
  }

  handleZweitstimmeAbgeben() {
    this.setState({ zweitstimmeenthaltung: false });
    this.setState({ zweitstimmeabgegeben: 1 });
    this.handleAbschluss();
  }

  handleZweitstimmeEnthalten() {
    this.setState({ zweitstimmeenthaltung: true });
    this.setState({ zweitstimmeabgegeben: 1 });
    this.handleAbschluss();
  }

  handleAbschluss() {
    this.vote();
    this.setState({ identity: false });
    this.setState({ erststimme: false });
    this.setState({ zweitstimme: false });
    this.setState({ abschluss: true });
  }
}
