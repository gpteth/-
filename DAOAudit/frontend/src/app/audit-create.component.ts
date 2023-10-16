import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup } from '@angular/forms';
import { AuditService } from '../audit.service';

@Component({
  selector: 'app-audit-create',
  templateUrl: './audit-create.component.html',
  styleUrls: ['./audit-create.component.css'],
})
export class AuditCreateComponent implements OnInit {
  auditForm: FormGroup;

  constructor(private formBuilder: FormBuilder, private auditService: AuditService) {}

  ngOnInit(): void {
    this.auditForm = this.formBuilder.group({
      name: [''],
      status: [''],
    });
  }

  onSubmit() {
    const auditData = this.auditForm.value;
    this.auditService.createAudit(auditData).subscribe((response) => {
      // Handle success or error here
      console.log('Audit created:', response);
    });
  }
}
