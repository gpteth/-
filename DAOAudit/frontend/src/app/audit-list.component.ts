import { Component, OnInit } from '@angular/core';
import { AuditService } from '../audit.service';

@Component({
  selector: 'app-audit-list',
  templateUrl: './audit-list.component.html',
  styleUrls: ['./audit-list.component.css'],
})
export class AuditListComponent implements OnInit {
  audits: any[] = [];

  constructor(private auditService: AuditService) {}

  ngOnInit(): void {
    this.loadAudits();
  }

  loadAudits() {
    this.auditService.getAudits().subscribe((audits) => {
      this.audits = audits;
    });
  }
}

